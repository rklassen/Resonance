mod prior;
mod probe;
mod receptor;
mod runtime;

use std::fmt::{Display, Formatter};

use crate::{
    alpha::{load_image, load_text, AlphaArtifact, AlphaProbeCache},
    beta::{run_beta_artifact, BetaRun},
};

pub use prior::{
    run_gamma_prior_ensemble_suite, GammaPriorAlignment, GammaPriorEnsembleSuite, GammaPriorRecord,
    GammaPriorSource,
};
pub use probe::{
    run_gamma_latent_sweep_suite, run_gamma_probe_suite, run_gamma_probe_validity_suite,
    GammaAxisValidityAssessment, GammaFailureMode, GammaFailureModeAssessment,
    GammaFailureModeDisposition, GammaLatentAxisStability, GammaLatentAxisSweep,
    GammaLatentAxisVariantRun, GammaLatentPromptVariant, GammaLatentSweepSuite, GammaProbeFamily,
    GammaProbeFamilyRun, GammaProbeSuite, GammaProbeValiditySuite,
};
pub use receptor::{
    run_gamma_receptor_bridge_suite, GammaReceptorBridgeSuite, GammaReceptorFamilyComparison,
};
pub use runtime::{
    run_gamma_dual_path_runtime, GammaDualPathRuntime, GammaNarrativePath, GammaObjectivePath,
};

#[derive(Clone, Debug, PartialEq)]
pub struct GammaRun {
    pub beta: BetaRun,
    pub probe_suite: GammaProbeSuite,
    pub latent_sweeps: GammaLatentSweepSuite,
    pub probe_validity: GammaProbeValiditySuite,
    pub prior_ensemble: GammaPriorEnsembleSuite,
    pub receptor_bridge: GammaReceptorBridgeSuite,
    pub dual_path_runtime: GammaDualPathRuntime,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaError {
    message: String,
}

impl GammaError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for GammaError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for GammaError {}

impl From<crate::alpha::AlphaError> for GammaError {
    fn from(error: crate::alpha::AlphaError) -> Self {
        Self::new(error.to_string())
    }
}

impl From<crate::beta::BetaError> for GammaError {
    fn from(error: crate::beta::BetaError) -> Self {
        Self::new(error.to_string())
    }
}

pub fn run_gamma_text(
    cache: &mut AlphaProbeCache,
    source: &str,
    text: &str,
) -> Result<GammaRun, GammaError> {
    run_gamma_artifact(cache, load_text(source, text))
}

pub fn run_gamma_image(
    cache: &mut AlphaProbeCache,
    source: &str,
    media_type: &str,
    bytes: &[u8],
) -> Result<GammaRun, GammaError> {
    run_gamma_artifact(cache, load_image(source, media_type, bytes))
}

fn run_gamma_artifact(
    cache: &mut AlphaProbeCache,
    artifact: AlphaArtifact,
) -> Result<GammaRun, GammaError> {
    let beta = run_beta_artifact(cache, artifact.clone())?;
    let probe_suite =
        run_gamma_probe_suite(cache, &artifact, &beta.embedding_probe, &beta.label_probe)?;
    let latent_sweeps = run_gamma_latent_sweep_suite(cache, &artifact)?;
    let probe_validity = run_gamma_probe_validity_suite(&latent_sweeps)?;
    let prior_ensemble = run_gamma_prior_ensemble_suite()?;
    let receptor_bridge = run_gamma_receptor_bridge_suite(&prior_ensemble, &beta.gain)?;
    let dual_path_runtime = run_gamma_dual_path_runtime(&beta)?;

    Ok(GammaRun {
        beta,
        probe_suite,
        latent_sweeps,
        probe_validity,
        prior_ensemble,
        receptor_bridge,
        dual_path_runtime,
    })
}
