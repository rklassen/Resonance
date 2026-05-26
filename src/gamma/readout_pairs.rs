use crate::{beta::BetaRun, SemanticError, SemanticResult};

use super::{
    GammaDisagreementLocalizer, GammaDualPathRuntime, GammaProjectionAgreement,
};

const DISAGREEMENT_THRESHOLD: f32 = f32::EPSILON;

pub(super) fn build_gamma_projection_pairs(
    beta: &BetaRun,
    dual_path: &GammaDualPathRuntime,
) -> SemanticResult<Vec<GammaProjectionAgreement>> {
    let probe_probe = pair(
        "probe-probe",
        mean_absolute_difference(&beta.embedding_probe.values, &beta.label_probe.values),
        GammaDisagreementLocalizer::Probe("embedding-probe-vs-label-probe".into()),
        "mean absolute difference between 16D embedding probe and 12D label probe (compared over 12 common dimensions)",
    );
    let semantic_vibes = pair(
        "semantic-vibes",
        mean_absolute_difference(&beta.label_probe.values, &beta.vibes.signed_12d),
        GammaDisagreementLocalizer::Transform("vibes-projection".into()),
        "mean absolute difference between 12D label probe and 12D vibes projection",
    );
    let vibes_receptor = pair(
        "vibes-receptor",
        mean_absolute_difference(
            &build_vibes_family_vector(beta, &dual_path.narrative_path.family_names)?,
            &dual_path.narrative_path.family_mean_vector,
        ),
        GammaDisagreementLocalizer::Transform("beta-receptor-bridge".into()),
        "mean absolute difference between vibes evidence-axis family summary and gamma narrative receptor-family summary",
    );
    let receptor_parcel = pair(
        "receptor-parcel",
        mean_absolute_difference(&beta.gain.vector, &beta.walk.state_after),
        GammaDisagreementLocalizer::Prior("receptor-gain-terms".into()),
        "mean absolute difference between 360D receptor gain vector and 360D post-walk parcel state",
    );
    let parcel_trajectory = GammaProjectionAgreement {
        name: "parcel-trajectory".into(),
        disagreement: mean_absolute_difference(
            &beta.walk.state_after,
            &dual_path.objective_path.runtime.state_after,
        ),
        localizer: Some(GammaDisagreementLocalizer::Operator("gamma-objective-runtime".into())),
        detail: "mean absolute difference between 360D beta walk state and 360D gamma objective runtime state; divergence is by design (gamma operator transforms parcel)".into(),
    };

    Ok(vec![
        probe_probe,
        semantic_vibes,
        vibes_receptor,
        receptor_parcel,
        parcel_trajectory,
    ])
}

fn pair(
    name: &str,
    disagreement: f32,
    localizer: GammaDisagreementLocalizer,
    detail: &str,
) -> GammaProjectionAgreement {
    GammaProjectionAgreement {
        name: name.into(),
        disagreement,
        localizer: if disagreement > DISAGREEMENT_THRESHOLD {
            Some(localizer)
        } else {
            None
        },
        detail: detail.into(),
    }
}

fn build_vibes_family_vector(
    beta: &BetaRun,
    family_names: &[String],
) -> SemanticResult<Vec<f32>> {
    family_names
        .iter()
        .map(|family| {
            let evidence = beta
                .gain
                .terms
                .iter()
                .filter(|term| &term.family == family)
                .flat_map(|term| term.evidence_axes.iter())
                .map(|axis| axis_value(&beta.vibes.signed_12d, axis))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    SemanticError::new(format!(
                        "gamma cross-projection readout found unsupported vibes axis for receptor family {}",
                        family
                    ))
                })?;
            Ok(evidence.iter().sum::<f32>() / evidence.len() as f32)
        })
        .collect()
}

fn axis_value(vibes: &[f32; 12], axis: &str) -> Option<f32> {
    let index = match axis {
        "valence" => 0,
        "arousal" => 1,
        "dominance" => 2,
        "warmth" => 3,
        "tension" => 4,
        "coherence" => 5,
        "novelty" => 6,
        "weight" => 7,
        "brightness" => 8,
        "depth" => 10,
        "motion" => 11,
        _ => return None,
    };
    Some(vibes[index])
}

fn mean_absolute_difference(left: &[f32], right: &[f32]) -> f32 {
    let width = left.len().min(right.len());
    if width == 0 {
        return 0.0;
    }
    left[..width].iter().zip(right[..width].iter()).map(|(a, b)| (a - b).abs()).sum::<f32>()
        / width as f32
}