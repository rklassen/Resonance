use crate::alpha::{run_configured_probe, AlphaArtifact, AlphaProbeCache, AlphaProbeRun, ProbeRecipe};
use crate::SemanticError;

use super::{
    GammaLatentAxisStability, GammaLatentAxisSweep, GammaLatentAxisVariantRun,
    GammaLatentPromptVariant, GammaLatentSweepSuite,
};

const LATENT_AXIS_MODEL_ID: &str = "model.gamma.latent-axis.v1";
const LATENT_AXIS_OUTPUT_CONTRACT: &str = "contract.payload.gamma.latent-axis";
const LATENT_AXIS_CAPABILITY: &str = "capability.latent-axis-sweep";
const LATENT_AXIS_OUTPUT_LEN: usize = 12;
const LATENT_AXIS_SCORE_FLOOR: f32 = 0.18;
const LATENT_AXIS_SPREAD_CEILING: f32 = 0.45;

struct GammaLatentAxisSpec {
    axis: &'static str,
    left_pole: &'static str,
    right_pole: &'static str,
}

pub fn run_gamma_latent_sweep_suite(
    cache: &mut AlphaProbeCache,
    artifact: &AlphaArtifact,
) -> crate::SemanticResult<GammaLatentSweepSuite> {
    let mut axes = Vec::new();
    for axis in latent_axis_specs() {
        let variants = latent_prompt_variants()
            .into_iter()
            .map(|(name, created)| {
                let recipe = latent_axis_recipe(&axis, name, created);
                let variant = GammaLatentPromptVariant {
                    name: name.into(),
                    prompt_id: recipe.prompt_id.clone(),
                };
                let run = run_configured_probe(cache, artifact, recipe);
                let score = latent_axis_score(&run);

                GammaLatentAxisVariantRun {
                    variant,
                    run,
                    score,
                }
            })
            .collect::<Vec<_>>();

        let mean_score =
            variants.iter().map(|variant| variant.score).sum::<f32>() / variants.len() as f32;
        let (min_score, max_score) = variants.iter().fold(
            (f32::INFINITY, f32::NEG_INFINITY),
            |(min_score, max_score), variant| {
                (min_score.min(variant.score), max_score.max(variant.score))
            },
        );
        let spread = max_score - min_score;
        let stability = classify_axis_stability(&axis, &variants, spread);

        axes.push(GammaLatentAxisSweep {
            axis: axis.axis.into(),
            left_pole: axis.left_pole.into(),
            right_pole: axis.right_pole.into(),
            model_id: LATENT_AXIS_MODEL_ID.into(),
            output_contract: LATENT_AXIS_OUTPUT_CONTRACT.into(),
            variants,
            mean_score,
            spread,
            stability,
        });
    }

    if axes.len() != 6 {
        return Err(SemanticError::new("gamma latent sweep suite must expose six axes"));
    }

    Ok(GammaLatentSweepSuite {
        axes,
    })
}

fn latent_axis_specs() -> Vec<GammaLatentAxisSpec> {
    vec![
        GammaLatentAxisSpec {
            axis: "cheap-premium",
            left_pole: "cheap",
            right_pole: "premium",
        },
        GammaLatentAxisSpec {
            axis: "organic-synthetic",
            left_pole: "organic",
            right_pole: "synthetic",
        },
        GammaLatentAxisSpec {
            axis: "calm-loud",
            left_pole: "calm",
            right_pole: "loud",
        },
        GammaLatentAxisSpec {
            axis: "earthy-neon",
            left_pole: "earthy",
            right_pole: "neon",
        },
        GammaLatentAxisSpec {
            axis: "threat-safety",
            left_pole: "threat",
            right_pole: "safety",
        },
        GammaLatentAxisSpec {
            axis: "approach-avoidance",
            left_pole: "approach",
            right_pole: "avoidance",
        },
    ]
}

fn latent_prompt_variants() -> Vec<(&'static str, crate::UtcMinute)> {
    vec![
        ("catalog", crate::UtcMinute(202605250211)),
        ("editorial", crate::UtcMinute(202605250212)),
        ("functional", crate::UtcMinute(202605250213)),
    ]
}

fn latent_axis_recipe(
    axis: &GammaLatentAxisSpec,
    variant_name: &str,
    created: crate::UtcMinute,
) -> ProbeRecipe {
    ProbeRecipe {
        probe_id: format!("probe-gamma-axis-{}-{}", axis.axis, variant_name),
        name: format!("gamma-latent-axis-{}-{}", axis.axis, variant_name),
        model_id: LATENT_AXIS_MODEL_ID.into(),
        prompt_id: format!("policy.gamma.latent-axis.{}.{}.v1", axis.axis, variant_name),
        output_contract: LATENT_AXIS_OUTPUT_CONTRACT.into(),
        capability: LATENT_AXIS_CAPABILITY.into(),
        output_len: LATENT_AXIS_OUTPUT_LEN,
        created,
    }
}

fn latent_axis_score(run: &AlphaProbeRun) -> f32 {
    let weights = [1.0, -0.75, 0.5, -0.25];
    let weighted_sum = run
        .values
        .iter()
        .enumerate()
        .map(|(index, value)| value * weights[index % weights.len()])
        .sum::<f32>();

    weighted_sum / run.values.len() as f32
}

fn classify_axis_stability(
    axis: &GammaLatentAxisSpec,
    variants: &[GammaLatentAxisVariantRun],
    spread: f32,
) -> GammaLatentAxisStability {
    let all_left = variants.iter().all(|variant| variant.score <= -LATENT_AXIS_SCORE_FLOOR);
    let all_right = variants.iter().all(|variant| variant.score >= LATENT_AXIS_SCORE_FLOOR);

    if spread <= LATENT_AXIS_SPREAD_CEILING {
        if all_left {
            return GammaLatentAxisStability::Stable {
                dominant_pole: axis.left_pole.into(),
            };
        }
        if all_right {
            return GammaLatentAxisStability::Stable {
                dominant_pole: axis.right_pole.into(),
            };
        }
    }

    GammaLatentAxisStability::Unstable
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alpha::load_text;

    fn variant_run(
        axis: &GammaLatentAxisSpec,
        variant_name: &'static str,
        score: f32,
    ) -> GammaLatentAxisVariantRun {
        let mut cache = AlphaProbeCache::default();
        let artifact = load_text(
            "artifact://gamma/test/latent-axis",
            "neutral latent-axis branch coverage artifact",
        );
        let created = latent_prompt_variants()
            .into_iter()
            .find_map(|(name, created)| (name == variant_name).then_some(created))
            .expect("test variant should exist");
        let recipe = latent_axis_recipe(axis, variant_name, created);
        let prompt_id = recipe.prompt_id.clone();
        let run = run_configured_probe(&mut cache, &artifact, recipe);

        GammaLatentAxisVariantRun {
            variant: GammaLatentPromptVariant {
                name: variant_name.into(),
                prompt_id,
            },
            run,
            score,
        }
    }

    #[test]
    fn classify_axis_stability_marks_stable_left() {
        let axis = GammaLatentAxisSpec {
            axis: "cheap-premium",
            left_pole: "cheap",
            right_pole: "premium",
        };
        let variants = vec![
            variant_run(&axis, "catalog", -0.23),
            variant_run(&axis, "editorial", -0.21),
            variant_run(&axis, "functional", -0.19),
        ];

        assert_eq!(
            classify_axis_stability(&axis, &variants, 0.04),
            GammaLatentAxisStability::Stable {
                dominant_pole: axis.left_pole.into(),
            }
        );
    }

    #[test]
    fn classify_axis_stability_marks_stable_right() {
        let axis = GammaLatentAxisSpec {
            axis: "cheap-premium",
            left_pole: "cheap",
            right_pole: "premium",
        };
        let variants = vec![
            variant_run(&axis, "catalog", 0.19),
            variant_run(&axis, "editorial", 0.21),
            variant_run(&axis, "functional", 0.23),
        ];

        assert_eq!(
            classify_axis_stability(&axis, &variants, 0.04),
            GammaLatentAxisStability::Stable {
                dominant_pole: axis.right_pole.into(),
            }
        );
    }

    #[test]
    fn classify_axis_stability_keeps_wide_consensus_unstable() {
        let axis = GammaLatentAxisSpec {
            axis: "cheap-premium",
            left_pole: "cheap",
            right_pole: "premium",
        };
        let variants = vec![
            variant_run(&axis, "catalog", 0.19),
            variant_run(&axis, "editorial", 0.28),
            variant_run(&axis, "functional", 0.32),
        ];

        assert_eq!(
            classify_axis_stability(&axis, &variants, LATENT_AXIS_SPREAD_CEILING + 0.01),
            GammaLatentAxisStability::Unstable
        );
    }
}