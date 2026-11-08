//! The clouds' fog over the transparent surfaces (`assets/shaders/clouds/fog.wesl`).
//!
//! The composite blends the clouds over the sky and the opaque objects; the transparent
//! ones, drawn after it, stood out of the fog unveiled: inside a cloud, the wires and the
//! propeller's disc stayed sharp over an aircraft half lost in it (`traversee_nuage`,
//! 2026-11-08). Their materials (`render::wires`, `render::own_normals`) bind the cloud
//! images and a buffer shared by all of them, written here once a frame, and veil their
//! colour with the fog in front of them:
//! - where the march of their pixel stopped at their own distance, on the aircraft behind
//!   them, with the light and transmittance of that march;
//! - where it went on behind them, through the sky, with the clouds' extinction at the
//!   eye: the fog between the eye and a surface a few metres away is the one around the
//!   eye. Out of the clouds there is none, whatever lies behind.

use bevy::asset::RenderAssetUsages;
use bevy::math::{DVec3, Vec3};
use bevy::prelude::*;
use bevy::render::render_resource::TextureUsages;
use bevy::render::storage::ShaderBuffer;
use escadrille_weather::clouds::CloudTextures;

use super::{CameraOnly, CloudTarget, Clouds, DISTANCE_FORMAT, TARGET_FORMAT};
use crate::flight::bridge::to_bevy;
use crate::flight::simulation::RenderOrigin;

/// What the transparent materials bind to take the clouds' fog: the cloud images after
/// the temporal pass (light and transmittance; where each march stopped), the same for
/// every sky, and the buffer of the shader's `CloudFog`: the clouds' extinction at the eye
/// (1/m) and the divisor of the images, 0 under a clear sky.
#[derive(Resource, Debug, Clone, Default)]
pub struct CloudFog {
    pub light: Handle<Image>,
    pub distance: Handle<Image>,
    pub fog: Handle<ShaderBuffer>,
}

/// The noise textures of the clouds, kept on the CPU: the density the game queries
/// (`escadrille_weather`, the reference of the shader's).
#[derive(Resource, Debug, Clone)]
pub struct CloudDensity(pub CloudTextures);

/// The cloud images, one texel until the camera's size is known, and the fog's buffer,
/// off.
pub(super) fn create(
    mut images: ResMut<Assets<Image>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut commands: Commands,
) {
    let mut image = |format| {
        let mut image = Image::new_target_texture(1, 1, format, None);
        // The temporal pass keeps a copy for the next frame.
        image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
        images.add(image)
    };
    let light = image(TARGET_FORMAT);
    let distance = image(DISTANCE_FORMAT);
    commands.insert_resource(CloudFog {
        light,
        distance,
        fog: buffers.add(ShaderBuffer::new(
            vec![0.0_f32; 2],
            RenderAssetUsages::default(),
        )),
    });
}

/// The clouds' extinction at the eye, and the divisor of the images.
#[expect(
    clippy::too_many_arguments,
    reason = "the clouds, their clock and origin, the fog's bindings, and the eye"
)]
pub(super) fn update(
    clouds: Res<Clouds>,
    density: Option<Res<CloudDensity>>,
    target: Res<CloudTarget>,
    fog: Res<CloudFog>,
    time: Res<Time>,
    origin: Option<Res<RenderOrigin>>,
    cameras: Query<&Transform, CameraOnly>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
) {
    let eye = cameras.iter().next().map_or(Vec3::ZERO, |t| t.translation);
    let origin = origin.map_or(DVec3::ZERO, |o| to_bevy(o.0));
    let uniforms = clouds.layer.uniforms(time.elapsed_secs_f64(), origin);
    let extinction = density.map_or(0.0, |d| d.0.extinction(&uniforms, eye));
    write(&mut buffers, &fog.fog, [extinction, target.divisor as f32]);
}

/// No fog under a clear sky.
pub(super) fn clear(fog: Option<Res<CloudFog>>, mut buffers: ResMut<Assets<ShaderBuffer>>) {
    if let Some(fog) = fog {
        write(&mut buffers, &fog.fog, [0.0, 0.0]);
    }
}

/// Writes the values into the buffer when they change: in place, so that the materials'
/// bind groups keep it.
fn write(buffers: &mut Assets<ShaderBuffer>, buffer: &Handle<ShaderBuffer>, values: [f32; 2]) {
    if buffers
        .get(buffer)
        .and_then(ShaderBuffer::cast_slice::<f32>)
        .is_some_and(|current| current != values)
        && let Some(mut buffer) = buffers.get_mut(buffer)
        && let Some(current) = buffer.cast_slice_mut::<f32>()
    {
        current.copy_from_slice(&values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_buffer_takes_the_values_in_place() {
        let mut buffers = Assets::<ShaderBuffer>::default();
        let handle = buffers.add(ShaderBuffer::new(
            vec![0.0_f32; 2],
            RenderAssetUsages::default(),
        ));
        write(&mut buffers, &handle, [0.09, 2.0]);
        let values = buffers.get(&handle).unwrap().cast_slice::<f32>().unwrap();
        assert_eq!(values, [0.09, 2.0]);
    }
}
