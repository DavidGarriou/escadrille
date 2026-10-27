//! Mip chains for the textures of loaded materials.
//!
//! Bevy 0.20 uploads a glTF's images with the single level they are stored with: it
//! generates levels only for render targets (`MipGenerationJobs`). The models' samplers
//! ask for trilinear filtering all the same, and the fine detail of the fabric (rib tapes
//! and stitching, 2.2 mm a texel) shimmers once a texel is smaller than a pixel. The
//! levels are computed here when a material and its textures arrive: colour averaged in
//! linear light, normals averaged then renormalised, other data averaged as stored.
//! KTX2 files with their levels made offline (plan `m3-modele-spad7.md`, B5 and B6) would
//! spare this work at load.

use bevy::image::{ImageFilterMode, ImageSampler};
use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;

/// Anisotropic filtering of the textures given a mip chain: the wings are mostly seen
/// edge-on. The highest level wgpu allows; it needs every filter linear.
pub const ANISOTROPY: u16 = 16;

/// How the texels of an image are averaged into the next level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Texels {
    /// Colour stored in sRGB: averaged in linear light.
    Srgb,
    /// Data stored as is (roughness, occlusion…).
    Linear,
    /// Tangent-space normals: averaged, then renormalised.
    Normal,
}

/// Fills the mip chains of the textures of each new material whose images have none.
pub fn fill_mip_chains(
    mut events: MessageReader<AssetEvent<StandardMaterial>>,
    materials: Res<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in events.read() {
        let (AssetEvent::Added { id } | AssetEvent::LoadedWithDependencies { id }) = event else {
            continue;
        };
        let Some(material) = materials.get(*id) else {
            continue;
        };
        let textures = [
            (&material.base_color_texture, false),
            (&material.emissive_texture, false),
            (&material.metallic_roughness_texture, false),
            (&material.occlusion_texture, false),
            (&material.normal_map_texture, true),
        ];
        for (handle, normals) in textures {
            let Some(handle) = handle else {
                continue;
            };
            // Look before taking it mutably: that marks the image modified, to upload again.
            if images.get(handle).is_some_and(needs_mip_chain) {
                let image = images.get_mut(handle).expect("the image was just found");
                fill_mip_chain(image.into_inner(), normals);
            }
        }
    }
}

/// An image of one level, in a format handled here, whose data the main world keeps.
fn needs_mip_chain(image: &Image) -> bool {
    let d = &image.texture_descriptor;
    d.mip_level_count == 1
        && d.size.depth_or_array_layers == 1
        && (d.size.width > 1 || d.size.height > 1)
        && matches!(
            d.format,
            TextureFormat::Rgba8Unorm | TextureFormat::Rgba8UnormSrgb
        )
        && image.data.is_some()
}

/// Replaces an image's single level by its full mip chain, and lets its sampler filter
/// between the levels.
pub fn fill_mip_chain(image: &mut Image, normals: bool) {
    if !needs_mip_chain(image) {
        return;
    }
    let texels = match image.texture_descriptor.format {
        _ if normals => Texels::Normal,
        TextureFormat::Rgba8UnormSrgb => Texels::Srgb,
        _ => Texels::Linear,
    };
    let (width, height) = (image.width(), image.height());
    let Some(data) = image.data.as_deref() else {
        return;
    };
    let (chain, levels) = mip_chain(data, width, height, texels);
    image.data = Some(chain);
    image.texture_descriptor.mip_level_count = levels;
    if let ImageSampler::Descriptor(sampler) = &mut image.sampler
        && [
            sampler.min_filter,
            sampler.mag_filter,
            sampler.mipmap_filter,
        ]
        .iter()
        .all(|f| *f == ImageFilterMode::Linear)
    {
        sampler.anisotropy_clamp = ANISOTROPY;
    }
}

/// Number of levels of a full mip chain, down to one texel.
pub fn level_count(width: u32, height: u32) -> u32 {
    32 - width.max(height).max(1).leading_zeros()
}

/// The mip chain of an RGBA8 image, its levels one after the other from the image itself
/// down to one texel (the layout wgpu expects for one layer), and their number. Each
/// texel of a level averages a 2 × 2 block of the one above, the last row or column
/// repeated where a size is odd.
pub fn mip_chain(data: &[u8], width: u32, height: u32, texels: Texels) -> (Vec<u8>, u32) {
    let levels = level_count(width, height);
    // Decoded once per code: four reads for each texel written.
    let table: [f32; 256] = std::array::from_fn(|c| {
        let v = c as f32 / 255.0;
        match texels {
            Texels::Srgb => srgb_to_linear(v),
            Texels::Linear => v,
            Texels::Normal => 2.0 * v - 1.0,
        }
    });
    let decode = |c: u8| table[usize::from(c)];
    let encode = |v: f32| -> u8 {
        let v = match texels {
            Texels::Srgb => linear_to_srgb(v),
            Texels::Linear => v,
            Texels::Normal => 0.5 * (v + 1.0),
        };
        (v.clamp(0.0, 1.0) * 255.0).round() as u8
    };
    let mut chain = data.to_vec();
    let (mut level, mut w, mut h) = (data.to_vec(), width as usize, height as usize);
    for _ in 1..levels {
        let (nw, nh) = ((w / 2).max(1), (h / 2).max(1));
        let mut next = vec![0u8; nw * nh * 4];
        for y in 0..nh {
            for x in 0..nw {
                let mut sum = [0.0f32; 4];
                for (dx, dy) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
                    let (sx, sy) = ((2 * x + dx).min(w - 1), (2 * y + dy).min(h - 1));
                    let texel = &level[(sy * w + sx) * 4..][..4];
                    for c in 0..3 {
                        sum[c] += decode(texel[c]);
                    }
                    sum[3] += f32::from(texel[3]) / 255.0;
                }
                let mut rgb = [sum[0] / 4.0, sum[1] / 4.0, sum[2] / 4.0];
                if texels == Texels::Normal {
                    let length = rgb.iter().map(|v| v * v).sum::<f32>().sqrt();
                    rgb = if length > 1e-6 {
                        rgb.map(|v| v / length)
                    } else {
                        [0.0, 0.0, 1.0]
                    };
                }
                let out = &mut next[(y * nw + x) * 4..][..4];
                for c in 0..3 {
                    out[c] = encode(rgb[c]);
                }
                out[3] = ((sum[3] / 4.0).clamp(0.0, 1.0) * 255.0).round() as u8;
            }
        }
        chain.extend_from_slice(&next);
        (level, w, h) = (next, nw, nh);
    }
    (chain, levels)
}

fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(v: f32) -> f32 {
    if v <= 0.003_130_8 {
        v * 12.92
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::RenderAssetUsages;
    use bevy::image::ImageSamplerDescriptor;
    use bevy::render::render_resource::{Extent3d, TextureDimension};

    use super::*;

    #[test]
    fn a_chain_runs_down_to_one_texel() {
        assert_eq!(level_count(4096, 4096), 13);
        assert_eq!(level_count(4, 2), 3);
        assert_eq!(level_count(1, 1), 1);
        let data: Vec<u8> = (0..4 * 2 * 4).map(|i| i as u8).collect();
        let (chain, levels) = mip_chain(&data, 4, 2, Texels::Linear);
        assert_eq!(levels, 3);
        assert_eq!(chain.len(), (8 + 2 + 1) * 4);
        assert_eq!(&chain[..data.len()], &data[..], "level 0 is the image");
    }

    #[test]
    fn colour_is_averaged_in_linear_light() {
        // Black and white side by side: half the light, not half the code.
        let data = [0, 0, 0, 255, 255, 255, 255, 255];
        let (chain, _) = mip_chain(&data, 2, 1, Texels::Srgb);
        assert_eq!(&chain[8..], &[188, 188, 188, 255]);
        let (chain, _) = mip_chain(&data, 2, 1, Texels::Linear);
        assert_eq!(&chain[8..], &[128, 128, 128, 255]);
    }

    #[test]
    fn normals_are_renormalised() {
        // Two normals tilted 45° either way average to a straight one, at full length.
        let tilt = (0.5 * (1.0 + std::f32::consts::FRAC_1_SQRT_2) * 255.0).round() as u8;
        let back = 255 - tilt;
        let data = [tilt, 128, tilt, 255, back, 128, tilt, 255];
        let (chain, _) = mip_chain(&data, 2, 1, Texels::Normal);
        assert_eq!(&chain[8..], &[128, 128, 255, 255]);
    }

    #[test]
    fn an_image_gets_its_levels_and_anisotropic_filtering() {
        let mut image = Image::new_fill(
            Extent3d {
                width: 8,
                height: 8,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[200, 180, 120, 255],
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
            mipmap_filter: ImageFilterMode::Linear,
            ..ImageSamplerDescriptor::linear()
        });
        fill_mip_chain(&mut image, false);
        assert_eq!(image.texture_descriptor.mip_level_count, 4);
        assert_eq!(image.data.as_ref().unwrap().len(), (64 + 16 + 4 + 1) * 4);
        // A flat colour stays itself on every level.
        assert!(
            image
                .data
                .as_ref()
                .unwrap()
                .chunks(4)
                .all(|t| t == [200, 180, 120, 255])
        );
        let ImageSampler::Descriptor(sampler) = &image.sampler else {
            panic!("a descriptor");
        };
        assert_eq!(sampler.anisotropy_clamp, ANISOTROPY);
        // Done once: a second call leaves the chain alone.
        let before = image.data.clone();
        fill_mip_chain(&mut image, false);
        assert_eq!(image.data, before);
    }
}
