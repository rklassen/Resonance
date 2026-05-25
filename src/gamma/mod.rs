mod probe;

use std::fmt::{Display, Formatter};

use crate::{
    alpha::{load_image, load_text, AlphaArtifact, AlphaProbeCache},
    beta::{run_beta_artifact, BetaRun},
};

pub use probe::{
    run_gamma_latent_sweep_suite, run_gamma_probe_suite, GammaLatentAxisStability,
    GammaLatentAxisSweep, GammaLatentAxisVariantRun, GammaLatentPromptVariant,
    GammaLatentSweepSuite, GammaProbeFamily, GammaProbeFamilyRun, GammaProbeSuite,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaConfig {
    pub extensions_disabled: bool,
}

impl Default for GammaConfig {
    fn default() -> Self {
        Self {
            extensions_disabled: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaRun {
    pub beta: BetaRun,
    pub probe_suite: GammaProbeSuite,
    pub latent_sweeps: GammaLatentSweepSuite,
    pub config: GammaConfig,
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
    run_gamma_with_config(cache, load_text(source, text), GammaConfig::default())
}

pub fn run_gamma_image(
    cache: &mut AlphaProbeCache,
    source: &str,
    media_type: &str,
    bytes: &[u8],
) -> Result<GammaRun, GammaError> {
    run_gamma_with_config(cache, load_image(source, media_type, bytes), GammaConfig::default())
}

pub fn run_gamma_text_with_config(
    cache: &mut AlphaProbeCache,
    source: &str,
    text: &str,
    config: GammaConfig,
) -> Result<GammaRun, GammaError> {
    run_gamma_with_config(cache, load_text(source, text), config)
}

fn run_gamma_with_config(
    cache: &mut AlphaProbeCache,
    artifact: AlphaArtifact,
    config: GammaConfig,
) -> Result<GammaRun, GammaError> {
    let beta = run_beta_artifact(cache, artifact.clone())?;
    let probe_suite =
        run_gamma_probe_suite(cache, &artifact, &beta.embedding_probe, &beta.label_probe, &config)?;
    let latent_sweeps = run_gamma_latent_sweep_suite(cache, &artifact, &config)?;

    Ok(GammaRun {
        beta,
        probe_suite,
        latent_sweeps,
        config,
    })
}
