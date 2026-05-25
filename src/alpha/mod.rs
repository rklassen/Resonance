mod artifact;
mod cache;
mod graph;
mod probe;
mod receptor;
mod snap;
mod totality;
mod trace;
mod vibes;
mod walk;

use std::fmt::{Display, Formatter};

pub use artifact::{hash_artifact, load_image, load_text, normalize_metadata, AlphaArtifact};
pub use cache::{AlphaCacheKey, AlphaProbeCache, CacheLookup, CacheStatus};
pub use graph::AlphaParcelGraph;
pub use probe::{
    run_configured_probe, run_embedding_probe, run_label_probe, AlphaProbeRun, ProbeRecipe,
};
pub use receptor::{mock_receptor_gain, AlphaGain, PARCEL_COUNT};
pub use snap::{
    AlphaSnapDocument, AlphaSnapEdgeGroup, AlphaSnapNode, AlphaSnapRegister, AlphaSnapType,
};
pub use trace::{assemble_trace_report, AlphaTraceReport};
pub use vibes::{project_to_vibes, AlphaVibes, AxisSpec, ProjectionValidation, VIBES_12D};
pub use walk::{laplacian_walk, AlphaWalk};

#[derive(Clone, Debug, PartialEq)]
pub struct AlphaRun {
    pub artifact: AlphaArtifact,
    pub embedding_probe: AlphaProbeRun,
    pub label_probe: AlphaProbeRun,
    pub vibes: AlphaVibes,
    pub gain: AlphaGain,
    pub graph: AlphaParcelGraph,
    pub walk: AlphaWalk,
    pub report: AlphaTraceReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaError {
    message: String,
}

impl AlphaError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for AlphaError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for AlphaError {}

pub fn run_alpha_text(
    cache: &mut AlphaProbeCache,
    source: &str,
    text: &str,
) -> Result<AlphaRun, AlphaError> {
    run_alpha(cache, load_text(source, text))
}

pub fn run_alpha_image(
    cache: &mut AlphaProbeCache,
    source: &str,
    media_type: &str,
    bytes: &[u8],
) -> Result<AlphaRun, AlphaError> {
    run_alpha(cache, load_image(source, media_type, bytes))
}

fn run_alpha(cache: &mut AlphaProbeCache, artifact: AlphaArtifact) -> Result<AlphaRun, AlphaError> {
    let embedding_probe = run_embedding_probe(cache, &artifact);
    let label_probe = run_label_probe(cache, &artifact);
    let vibes = project_to_vibes(&embedding_probe, &label_probe)?;
    let gain = mock_receptor_gain(&vibes)?;
    let graph = AlphaParcelGraph::mock_360();
    let walk = laplacian_walk(&graph, &vibes, &gain)?;
    let report = assemble_trace_report(
        &artifact,
        &embedding_probe,
        &label_probe,
        &vibes,
        &gain,
        &graph,
        &walk,
    )?;

    Ok(AlphaRun {
        artifact,
        embedding_probe,
        label_probe,
        vibes,
        gain,
        graph,
        walk,
        report,
    })
}
