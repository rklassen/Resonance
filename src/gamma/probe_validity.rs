use crate::alpha::{AlphaArtifact, AlphaProbeCache};
use crate::SemanticError;

use super::probe_validity_checks::{
    domain_mismatch_failure, label_collision_failure, model_disagreement_failure,
    neighborhood_instability_failure, AxisSignature,
};
use super::{
    GammaLatentAxisStability, GammaLatentAxisSweep, GammaLatentSweepSuite,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GammaFailureMode {
    PromptSensitivity,
    ModelDisagreement,
    EmbeddingNeighborhoodInstability,
    LabelCollision,
    DomainMismatch,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GammaFailureModeDisposition {
    Clear,
    Observed,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaFailureModeAssessment {
    pub mode: GammaFailureMode,
    pub disposition: GammaFailureModeDisposition,
    pub detail: String,
    pub required_follow_up: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaAxisValidityAssessment {
    pub axis: String,
    pub high_confidence_eligible: bool,
    pub failure_modes: Vec<GammaFailureModeAssessment>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaProbeValiditySuite {
    pub axes: Vec<GammaAxisValidityAssessment>,
}

pub fn run_gamma_probe_validity_suite(
    cache: &mut AlphaProbeCache,
    artifact: &AlphaArtifact,
    latent_sweeps: &GammaLatentSweepSuite,
) -> crate::SemanticResult<GammaProbeValiditySuite> {
    let label_signatures = latent_sweeps
        .axes
        .iter()
        .map(|axis| AxisSignature {
            axis: axis.axis.clone(),
            scores: axis.variants.iter().map(|variant| variant.score).collect(),
        })
        .collect::<Vec<_>>();
    let axes = latent_sweeps
        .axes
        .iter()
        .map(|axis| {
            let failure_modes = vec![
                prompt_sensitivity_failure(axis),
                model_disagreement_failure(cache, artifact, axis)?,
                neighborhood_instability_failure(axis),
                label_collision_failure(axis, &label_signatures),
                domain_mismatch_failure(artifact, axis),
            ];
            let high_confidence_eligible = failure_modes
                .iter()
                .all(|failure| failure.disposition == GammaFailureModeDisposition::Clear);

            Ok(GammaAxisValidityAssessment {
                axis: axis.axis.clone(),
                high_confidence_eligible,
                failure_modes,
            })
        })
        .collect::<Result<Vec<_>, SemanticError>>()?;

    if axes.len() != latent_sweeps.axes.len() {
        return Err(SemanticError::new(
            "gamma probe validity suite must evaluate every latent axis",
        ));
    }

    Ok(GammaProbeValiditySuite {
        axes,
    })
}

pub fn prompt_sensitivity_failure(axis: &GammaLatentAxisSweep) -> GammaFailureModeAssessment {
    match axis.stability {
        GammaLatentAxisStability::Stable { .. } => GammaFailureModeAssessment {
            mode: GammaFailureMode::PromptSensitivity,
            disposition: GammaFailureModeDisposition::Clear,
            detail: format!(
                "latent-axis prompt variants remained stable for {} with spread {:.6}",
                axis.axis, axis.spread
            ),
            required_follow_up: None,
        },
        GammaLatentAxisStability::Unstable => GammaFailureModeAssessment {
            mode: GammaFailureMode::PromptSensitivity,
            disposition: GammaFailureModeDisposition::Observed,
            detail: format!(
                "latent-axis prompt variants were unstable for {} with spread {:.6}",
                axis.axis, axis.spread
            ),
            required_follow_up: Some(format!(
                "re-observe {} after prompt-variant stability improves",
                axis.axis
            )),
        },
    }
}
pub(super) fn clear_failure(mode: GammaFailureMode, detail: String) -> GammaFailureModeAssessment {
    GammaFailureModeAssessment {
        mode,
        disposition: GammaFailureModeDisposition::Clear,
        detail,
        required_follow_up: None,
    }
}

pub(super) fn observed_failure(
    mode: GammaFailureMode,
    detail: String,
    required_follow_up: String,
) -> GammaFailureModeAssessment {
    GammaFailureModeAssessment {
        mode,
        disposition: GammaFailureModeDisposition::Observed,
        detail,
        required_follow_up: Some(required_follow_up),
    }
}