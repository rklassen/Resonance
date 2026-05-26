use crate::alpha::{run_configured_probe, AlphaArtifact, AlphaProbeCache, ProbeRecipe};
use crate::SemanticError;

use super::probe_validity::{
    clear_failure, observed_failure, GammaFailureMode, GammaFailureModeAssessment,
};
use super::GammaLatentAxisSweep;

const ALT_LATENT_AXIS_MODEL_ID: &str = "model.gamma.latent-axis.alt.v1";
const ALT_LATENT_AXIS_PROBE_PREFIX: &str = "probe-gamma-axis-alt";
const MODEL_DISAGREEMENT_THRESHOLD: f32 = 0.085;
const NEIGHBORHOOD_OVERLAP_FLOOR: f32 = 0.5;
const LABEL_COLLISION_THRESHOLD: f32 = 0.96;
const DOMAIN_SIGNAL_FLOOR: f32 = 0.015;
const DOMAIN_TEXT_CHAR_FLOOR: usize = 24;
const DOMAIN_IMAGE_BYTE_FLOOR: usize = 32;
const NEIGHBORHOOD_WIDTH: usize = 4;

pub(super) fn model_disagreement_failure(
    cache: &mut AlphaProbeCache,
    artifact: &AlphaArtifact,
    axis: &GammaLatentAxisSweep,
) -> crate::SemanticResult<GammaFailureModeAssessment> {
    let alternate_scores = axis
        .variants
        .iter()
        .map(|variant| {
            let run = run_configured_probe(
                cache,
                artifact,
                alternate_model_recipe(&axis.axis, &variant.variant.name, variant.run.values.len()),
            );
            Ok(latent_axis_score(&run.values))
        })
        .collect::<Result<Vec<_>, SemanticError>>()?;
    let primary_mean = axis.mean_score;
    let alternate_mean = alternate_scores.iter().sum::<f32>() / alternate_scores.len() as f32;
    let delta = (primary_mean - alternate_mean).abs();
    let sign_conflict = primary_mean.signum() != alternate_mean.signum()
        && primary_mean.abs() >= DOMAIN_SIGNAL_FLOOR
        && alternate_mean.abs() >= DOMAIN_SIGNAL_FLOOR;
    if delta <= MODEL_DISAGREEMENT_THRESHOLD && !sign_conflict {
        return Ok(clear_failure(
            GammaFailureMode::ModelDisagreement,
            format!(
                "alternate latent-axis model agreed for {} with mean delta {:.6}",
                axis.axis, delta
            ),
        ));
    }

    Ok(observed_failure(
        GammaFailureMode::ModelDisagreement,
        format!(
            "alternate latent-axis model disagreed for {} with mean delta {:.6}",
            axis.axis, delta
        ),
        format!(
            "inspect {} across additional latent-axis models before promoting model agreement",
            axis.axis
        ),
    ))
}

pub(super) fn neighborhood_instability_failure(
    axis: &GammaLatentAxisSweep,
) -> GammaFailureModeAssessment {
    let overlaps = pairwise_variant_overlaps(axis);
    let min_overlap = overlaps.into_iter().fold(1.0_f32, f32::min);
    if min_overlap >= NEIGHBORHOOD_OVERLAP_FLOOR {
        return clear_failure(
            GammaFailureMode::EmbeddingNeighborhoodInstability,
            format!(
                "latent-axis replay neighborhood remained stable for {} with minimum overlap {:.6}",
                axis.axis, min_overlap
            ),
        );
    }

    observed_failure(
        GammaFailureMode::EmbeddingNeighborhoodInstability,
        format!(
            "latent-axis replay neighborhood drifted for {} with minimum overlap {:.6}",
            axis.axis, min_overlap
        ),
        format!(
            "re-observe {} after neighborhood overlap is raised above {:.2}",
            axis.axis, NEIGHBORHOOD_OVERLAP_FLOOR
        ),
    )
}

pub(super) fn label_collision_failure(
    axis: &GammaLatentAxisSweep,
    label_signatures: &[AxisSignature],
) -> GammaFailureModeAssessment {
    let subject = label_signatures
        .iter()
        .find(|signature| signature.axis == axis.axis)
        .expect("axis signature should exist");
    let max_similarity = label_signatures
        .iter()
        .filter(|candidate| candidate.axis != axis.axis)
        .map(|candidate| cosine_similarity(&subject.scores, &candidate.scores))
        .fold(-1.0_f32, f32::max);
    if max_similarity < LABEL_COLLISION_THRESHOLD {
        return clear_failure(
            GammaFailureMode::LabelCollision,
            format!(
                "latent-axis label signature remained distinct for {} with max similarity {:.6}",
                axis.axis, max_similarity
            ),
        );
    }

    observed_failure(
        GammaFailureMode::LabelCollision,
        format!(
            "latent-axis label signature collided for {} with max similarity {:.6}",
            axis.axis, max_similarity
        ),
        format!(
            "separate {} from the most similar latent-axis label before promoting label stability",
            axis.axis
        ),
    )
}

pub(super) fn domain_mismatch_failure(
    artifact: &AlphaArtifact,
    axis: &GammaLatentAxisSweep,
) -> GammaFailureModeAssessment {
    let mean_abs_signal = axis
        .variants
        .iter()
        .map(|variant| variant.score.abs())
        .sum::<f32>()
        / axis.variants.len() as f32;
    let domain_supported = match artifact.normalized_text.as_ref() {
        Some(text) => text.chars().count() >= DOMAIN_TEXT_CHAR_FLOOR,
        None if artifact.media_type.starts_with("image/") => {
            artifact.normalized_bytes.len() >= DOMAIN_IMAGE_BYTE_FLOOR
        }
        None => false,
    };
    if domain_supported && mean_abs_signal >= DOMAIN_SIGNAL_FLOOR {
        return clear_failure(
            GammaFailureMode::DomainMismatch,
            format!(
                "probe-to-artifact domain fit remained acceptable for {} with mean absolute signal {:.6}",
                axis.axis, mean_abs_signal
            ),
        );
    }

    observed_failure(
        GammaFailureMode::DomainMismatch,
        format!(
            "probe-to-artifact domain fit is weak for {} with mean absolute signal {:.6}",
            axis.axis, mean_abs_signal
        ),
        format!(
            "re-observe {} after adding stronger artifact-domain evidence or a better-matched probe",
            axis.axis
        ),
    )
}

pub(super) struct AxisSignature {
    pub axis: String,
    pub scores: Vec<f32>,
}

fn alternate_model_recipe(axis: &str, variant_name: &str, output_len: usize) -> ProbeRecipe {
    ProbeRecipe {
        probe_id: format!("{}-{}-{}", ALT_LATENT_AXIS_PROBE_PREFIX, axis, variant_name),
        name: format!("gamma-latent-axis-alt-{}-{}", axis, variant_name),
        model_id: ALT_LATENT_AXIS_MODEL_ID.into(),
        prompt_id: format!("policy.gamma.latent-axis.{}.{}.v1", axis, variant_name),
        output_contract: "contract.payload.gamma.latent-axis".into(),
        capability: "capability.latent-axis-sweep".into(),
        output_len,
        created: crate::UtcMinute(202605250214),
    }
}

fn latent_axis_score(values: &[f32]) -> f32 {
    let weights = [1.0, -0.75, 0.5, -0.25];
    let weighted_sum = values
        .iter()
        .enumerate()
        .map(|(index, value)| value * weights[index % weights.len()])
        .sum::<f32>();

    weighted_sum / values.len() as f32
}

fn pairwise_variant_overlaps(axis: &GammaLatentAxisSweep) -> Vec<f32> {
    let neighborhoods = axis
        .variants
        .iter()
        .map(|variant| top_indices(&variant.run.values))
        .collect::<Vec<_>>();
    let mut overlaps = Vec::new();
    for left in 0..neighborhoods.len() {
        for right in (left + 1)..neighborhoods.len() {
            overlaps.push(jaccard_overlap(&neighborhoods[left], &neighborhoods[right]));
        }
    }
    overlaps
}

fn top_indices(values: &[f32]) -> Vec<usize> {
    let mut indexed = values
        .iter()
        .enumerate()
        .map(|(index, value)| (index, value.abs()))
        .collect::<Vec<_>>();
    indexed.sort_by(|left, right| right.1.partial_cmp(&left.1).unwrap());
    indexed
        .into_iter()
        .take(NEIGHBORHOOD_WIDTH.min(values.len()))
        .map(|(index, _)| index)
        .collect()
}

fn jaccard_overlap(left: &[usize], right: &[usize]) -> f32 {
    let intersection = left.iter().filter(|index| right.contains(index)).count();
    let union = left.len() + right.len() - intersection;
    if union == 0 {
        1.0
    } else {
        intersection as f32 / union as f32
    }
}

fn cosine_similarity(left: &[f32], right: &[f32]) -> f32 {
    let width = left.len().min(right.len());
    let dot = left[..width]
        .iter()
        .zip(right[..width].iter())
        .map(|(a, b)| a * b)
        .sum::<f32>();
    let left_norm = left[..width].iter().map(|value| value * value).sum::<f32>().sqrt();
    let right_norm = right[..width].iter().map(|value| value * value).sum::<f32>().sqrt();
    if left_norm == 0.0 || right_norm == 0.0 {
        0.0
    } else {
        dot / (left_norm * right_norm)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alpha::load_text;
    use crate::{
        GammaFailureModeDisposition, GammaLatentAxisStability, GammaLatentAxisVariantRun,
        GammaLatentPromptVariant,
    };

    fn unstable_axis() -> GammaLatentAxisSweep {
        let artifact = load_text(
            "artifact://gamma/test/validity",
            "warm reflective corridor with enough text for domain-fit evidence",
        );
        let run = run_configured_probe(
            &mut AlphaProbeCache::default(),
            &artifact,
            ProbeRecipe {
                probe_id: "probe-test-axis".into(),
                name: "probe-test-axis".into(),
                model_id: "model.test.axis.v1".into(),
                prompt_id: "policy.test.axis.v1".into(),
                output_contract: "contract.payload.gamma.latent-axis".into(),
                capability: "capability.latent-axis-sweep".into(),
                output_len: 12,
                created: crate::UtcMinute(202605250214),
            },
        );
        GammaLatentAxisSweep {
            axis: "cheap-premium".into(),
            left_pole: "cheap".into(),
            right_pole: "premium".into(),
            model_id: "model.gamma.latent-axis.v1".into(),
            output_contract: "contract.payload.gamma.latent-axis".into(),
            variants: vec![
                GammaLatentAxisVariantRun {
                    variant: GammaLatentPromptVariant {
                        name: "catalog".into(),
                        prompt_id: "policy.gamma.latent-axis.cheap-premium.catalog.v1".into(),
                    },
                    run: run.clone(),
                    score: 0.04,
                },
                GammaLatentAxisVariantRun {
                    variant: GammaLatentPromptVariant {
                        name: "editorial".into(),
                        prompt_id: "policy.gamma.latent-axis.cheap-premium.editorial.v1".into(),
                    },
                    run,
                    score: -0.01,
                },
            ],
            mean_score: 0.015,
            spread: 0.05,
            stability: GammaLatentAxisStability::Unstable,
        }
    }

    #[test]
    fn domain_mismatch_uses_artifact_fit_evidence() {
        let artifact = load_text("artifact://gamma/test/domain", "tiny");
        let assessment = domain_mismatch_failure(&artifact, &unstable_axis());

        assert_eq!(assessment.mode, GammaFailureMode::DomainMismatch);
        assert_eq!(assessment.disposition, GammaFailureModeDisposition::Observed);
        assert!(assessment.required_follow_up.is_some());
    }

    #[test]
    fn label_collision_marks_distinct_signatures_clear() {
        let axis = unstable_axis();
        let signatures = vec![
            AxisSignature {
                axis: "cheap-premium".into(),
                scores: vec![0.04, -0.01, 0.03],
            },
            AxisSignature {
                axis: "organic-synthetic".into(),
                scores: vec![-0.4, 0.5, -0.6],
            },
        ];
        let assessment = label_collision_failure(&axis, &signatures);

        assert_eq!(assessment.mode, GammaFailureMode::LabelCollision);
        assert_eq!(assessment.disposition, GammaFailureModeDisposition::Clear);
    }
}