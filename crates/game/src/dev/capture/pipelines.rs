//! The pipelines the render world fails to build, reported to the capture: an image
//! rendered without one of its shaders would pass the review and lie.
//!
//! Bevy gives up on a shader that does not compile, but retries forever a pipeline whose
//! shader, or one of its imports, is not loaded yet: a wrong WESL import is only a warning
//! in the log, and the material is never drawn. The capture fails at once on the first
//! kind, and on the second if it is still there when the image is due.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use bevy::prelude::*;
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_resource::{CachedPipelineState, PipelineCache, PipelineDescriptor};
use bevy::render::{Render, RenderApp, RenderSystems};
use bevy::shader::ShaderCacheError;

/// A pipeline that does not build.
#[derive(Debug, Clone, PartialEq)]
pub struct PipelineFailure {
    /// The pipeline's label: Bevy's material pipelines share theirs, the shaders tell them
    /// apart.
    pub pipeline: String,
    /// The shader of each stage, as `stage path`.
    pub shaders: Vec<String>,
    pub error: String,
    /// Bevy gave up on it; otherwise it retries, and the failure may still clear.
    pub fatal: bool,
}

impl PipelineFailure {
    fn new(id: usize, descriptor: &PipelineDescriptor, error: &ShaderCacheError) -> Self {
        let (label, stages) = match descriptor {
            PipelineDescriptor::RenderPipelineDescriptor(d) => {
                let mut stages = vec![("vertex", &d.vertex.shader)];
                stages.extend(d.fragment.as_ref().map(|f| ("fragment", &f.shader)));
                (&d.label, stages)
            }
            PipelineDescriptor::ComputePipelineDescriptor(d) => {
                (&d.label, vec![("compute", &d.shader)])
            }
            PipelineDescriptor::MeshPipelineDescriptor(d) => {
                let mut stages: Vec<_> = d.task.iter().map(|t| ("task", &t.shader)).collect();
                stages.push(("mesh", &d.mesh.shader));
                stages.extend(d.fragment.as_ref().map(|f| ("fragment", &f.shader)));
                (&d.label, stages)
            }
        };
        let (error, fatal) = match error {
            ShaderCacheError::ShaderNotLoaded(_) => ("a shader never loaded".to_owned(), false),
            ShaderCacheError::ShaderImportNotYetAvailable => (
                "a shader import never resolved (the warning above names it)".to_owned(),
                false,
            ),
            ShaderCacheError::ProcessShaderError(text) => {
                (format!("a shader does not compile:\n{text}"), true)
            }
            ShaderCacheError::CreateShaderModule(text) => {
                (format!("the GPU rejected a shader module: {text}"), true)
            }
        };
        Self {
            pipeline: label
                .as_deref()
                .map_or_else(|| format!("#{id}"), str::to_owned),
            shaders: stages
                .into_iter()
                .map(|(stage, shader)| match shader.path() {
                    Some(path) => format!("{stage} {path}"),
                    None => format!("{stage} {}", shader.id()),
                })
                .collect(),
            error,
            fatal,
        }
    }
}

impl fmt::Display for PipelineFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pipeline `{}` ({}) did not build: {}",
            self.pipeline,
            self.shaders.join(", "),
            self.error
        )
    }
}

/// The pipelines that do not build, by index in the render world's cache; one leaves the
/// list once it builds. Inserted by the capture, shared by both worlds.
#[derive(Resource, Debug, Clone, Default)]
pub struct PipelineFailures(Arc<Mutex<BTreeMap<usize, PipelineFailure>>>);

impl PipelineFailures {
    /// The failures now, those Bevy gave up on first. The variants of a material's
    /// pipeline fail alike, and count once.
    pub fn current(&self) -> Vec<PipelineFailure> {
        let mut failures: Vec<_> = self.lock().values().cloned().collect();
        failures.sort_by(|a, b| {
            (!a.fatal, &a.pipeline, &a.shaders, &a.error).cmp(&(
                !b.fatal,
                &b.pipeline,
                &b.shaders,
                &b.error,
            ))
        });
        failures.dedup();
        failures
    }

    fn lock(&self) -> MutexGuard<'_, BTreeMap<usize, PipelineFailure>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl ExtractResource<RenderApp> for PipelineFailures {
    type Source = Self;

    fn extract_resource(source: &Self::Source) -> Self {
        source.clone()
    }
}

pub fn build(app: &mut App) {
    app.add_plugins(ExtractResourcePlugin::<PipelineFailures>::default());
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };
    render_app.add_systems(
        Render,
        // After the cache processed its queue, in `RenderSystems::Render`.
        record_failures
            .run_if(resource_exists::<PipelineFailures>)
            .in_set(RenderSystems::Cleanup),
    );
}

fn record_failures(cache: Res<PipelineCache>, failures: Res<PipelineFailures>) {
    record(&cache, &mut failures.lock());
}

/// Brings `failures` up to date with the states of the cache's pipelines.
fn record(cache: &PipelineCache, failures: &mut BTreeMap<usize, PipelineFailure>) {
    for (id, pipeline) in cache.pipelines().enumerate() {
        match &pipeline.state {
            CachedPipelineState::Ok(_) => {
                failures.remove(&id);
            }
            CachedPipelineState::Err(error) => {
                failures.insert(id, PipelineFailure::new(id, &pipeline.descriptor, error));
            }
            // Also between two attempts of a retried failure, which stays on the list.
            CachedPipelineState::Queued | CachedPipelineState::Creating(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::asset::uuid::Uuid;
    use bevy::render::render_resource::ComputePipelineDescriptor;
    use bevy::render::renderer::initialize_renderer;
    use bevy::render::settings::{Backends, RenderResources, WgpuSettings};
    use bevy::shader::Shader;

    use super::*;

    /// A pipeline cache on the GPU, which builds its pipelines without a task pool.
    fn cache() -> PipelineCache {
        let settings = WgpuSettings::default();
        let backends = settings.backends.unwrap_or(Backends::all());
        let RenderResources(device, ..) =
            bevy::tasks::block_on(initialize_renderer(backends, None, &settings));
        PipelineCache::new(device, true)
    }

    /// Queues a compute pipeline on a WESL shader, as the game's material does.
    fn queue(cache: &mut PipelineCache, n: u128, source: &'static str) {
        let shader = Handle::<Shader>::from(Uuid::from_u128(n));
        cache.set_shader(
            shader.id(),
            Shader::from_wesl(source, format!("shaders/test_{n}.wesl")),
        );
        cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some(format!("test_{n}").into()),
            shader,
            ..default()
        });
    }

    /// Runs the cache for a few frames, recording after each one as the render world does.
    fn frames(cache: &mut PipelineCache, failures: &mut BTreeMap<usize, PipelineFailure>) {
        for _ in 0..6 {
            cache.process_queue();
            record(cache, failures);
        }
    }

    #[test]
    fn a_wrong_import_stays_a_failure_and_a_compile_error_is_fatal() {
        let mut cache = cache();
        queue(
            &mut cache,
            1,
            "import nowhere::helpers::value;\n@compute @workgroup_size(1) fn main() { _ = value(); }",
        );
        queue(
            &mut cache,
            2,
            "@compute @workgroup_size(1) fn main() { _ = ; }",
        );
        queue(&mut cache, 3, "@compute @workgroup_size(1) fn main() {}");
        let mut failures = BTreeMap::new();
        frames(&mut cache, &mut failures);

        let failed: Vec<_> = failures.values().collect();
        assert_eq!(failed.len(), 2, "{failed:#?}");
        assert_eq!(failed[0].pipeline, "test_1");
        assert!(!failed[0].fatal, "Bevy retries an unresolved import");
        assert!(failed[0].error.contains("import"), "{}", failed[0]);
        assert_eq!(failed[0].shaders.len(), 1);
        assert!(
            failed[0].shaders[0].starts_with("compute "),
            "{}",
            failed[0]
        );
        assert_eq!(failed[1].pipeline, "test_2");
        assert!(failed[1].fatal, "a syntax error is final");
    }

    #[test]
    fn a_failure_clears_once_the_import_arrives() {
        let mut cache = cache();
        queue(
            &mut cache,
            1,
            "import escadrille_test::helpers::value;\n@compute @workgroup_size(1) fn main() { _ = value(); }",
        );
        let mut failures = BTreeMap::new();
        frames(&mut cache, &mut failures);
        assert_eq!(failures.len(), 1, "{failures:#?}");

        cache.set_shader(
            Handle::<Shader>::from(Uuid::from_u128(10)).id(),
            Shader::from_wesl(
                "fn value() -> f32 { return 1.0; }",
                "embedded://escadrille_test/helpers.wesl",
            ),
        );
        frames(&mut cache, &mut failures);
        assert!(failures.is_empty(), "{failures:#?}");
    }

    #[test]
    fn fatal_failures_come_first_and_variants_count_once() {
        let failure = |pipeline: &str, fatal| PipelineFailure {
            pipeline: pipeline.to_owned(),
            shaders: vec!["fragment shaders/test.wesl".to_owned()],
            error: "a shader never loaded".to_owned(),
            fatal,
        };
        let failures = PipelineFailures::default();
        failures.lock().extend([
            (0, failure("variant", false)),
            (1, failure("broken", true)),
            (2, failure("variant", false)),
        ]);
        assert_eq!(
            failures.current(),
            [failure("broken", true), failure("variant", false)]
        );
    }

    #[test]
    fn a_failure_names_its_pipeline_and_shaders() {
        let failure = PipelineFailure {
            pipeline: "transparent_mesh_pipeline".to_owned(),
            shaders: vec![
                "vertex shaders/clouds/composite.wesl".to_owned(),
                "fragment shaders/clouds/composite.wesl".to_owned(),
            ],
            error: "a shader import never resolved".to_owned(),
            fatal: false,
        };
        assert_eq!(
            failure.to_string(),
            "pipeline `transparent_mesh_pipeline` (vertex shaders/clouds/composite.wesl, \
             fragment shaders/clouds/composite.wesl) did not build: a shader import never resolved"
        );
    }
}
