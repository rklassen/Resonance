mod disagreement;
mod fixtures;
mod graph;
mod receptor;
mod trace;
mod walk;

use std::fmt::{Display, Formatter};

use crate::alpha::{
    load_image, load_text, project_to_vibes, run_embedding_probe, run_label_probe, AlphaArtifact,
    AlphaProbeCache, AlphaProbeRun, AlphaVibes,
};

pub use disagreement::BetaDisagreement;
pub use fixtures::{
    load_public_fixtures, BetaAtlasFixture, BetaFixtureEdge, BetaFixtureGraph, BetaFixtureParcel,
    BetaFixturePrior, BetaPublicFixtures,
};
pub use graph::BetaParcelGraph;
pub use receptor::{real_receptor_gain, BetaGain, BetaGainTerm};
pub use trace::BetaTraceReport;
pub use walk::{laplacian_runtime, BetaWalk};

#[derive(Clone, Debug, PartialEq)]
pub struct BetaRun {
    pub artifact: AlphaArtifact,
    pub embedding_probe: AlphaProbeRun,
    pub label_probe: AlphaProbeRun,
    pub vibes: AlphaVibes,
    pub gain: BetaGain,
    pub graph: BetaParcelGraph,
    pub walk: BetaWalk,
    pub disagreement: BetaDisagreement,
    pub report: BetaTraceReport,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BetaError {
    message: String,
}

impl BetaError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for BetaError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for BetaError {}

impl From<crate::alpha::AlphaError> for BetaError {
    fn from(error: crate::alpha::AlphaError) -> Self {
        Self::new(error.to_string())
    }
}

pub fn run_beta_text(
    cache: &mut AlphaProbeCache,
    source: &str,
    text: &str,
) -> Result<BetaRun, BetaError> {
    run_beta_artifact(cache, load_text(source, text))
}

pub fn run_beta_image(
    cache: &mut AlphaProbeCache,
    source: &str,
    media_type: &str,
    bytes: &[u8],
) -> Result<BetaRun, BetaError> {
    run_beta_artifact(cache, load_image(source, media_type, bytes))
}

pub(crate) fn run_beta_artifact(
    cache: &mut AlphaProbeCache,
    artifact: AlphaArtifact,
) -> Result<BetaRun, BetaError> {
    let fixtures = load_public_fixtures()?;
    let embedding_probe = run_embedding_probe(cache, &artifact);
    let label_probe = run_label_probe(cache, &artifact);
    let vibes = project_to_vibes(&embedding_probe, &label_probe)?;
    let gain = real_receptor_gain(fixtures, &vibes)?;
    let graph = BetaParcelGraph::from_public_fixtures(fixtures)?;
    let walk = laplacian_runtime(&graph, &vibes, &gain)?;
    let disagreement =
        BetaDisagreement::measure(fixtures, &embedding_probe, &label_probe, &gain, &graph, &walk)?;
    let report = trace::assemble_trace_report(
        fixtures,
        &artifact,
        &embedding_probe,
        &label_probe,
        &vibes,
        &gain,
        &graph,
        &walk,
        &disagreement,
    )?;

    Ok(BetaRun {
        artifact,
        embedding_probe,
        label_probe,
        vibes,
        gain,
        graph,
        walk,
        disagreement,
        report,
    })
}
