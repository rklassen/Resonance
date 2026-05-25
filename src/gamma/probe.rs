use crate::alpha::{
    run_configured_probe, AlphaArtifact, AlphaProbeCache, AlphaProbeRun, ProbeRecipe,
};

use super::GammaError;

const LATENT_AXIS_MODEL_ID: &str = "model.gamma.latent-axis.v1";
const LATENT_AXIS_OUTPUT_CONTRACT: &str = "contract.payload.gamma.latent-axis";
const LATENT_AXIS_CAPABILITY: &str = "capability.latent-axis-sweep";
const LATENT_AXIS_OUTPUT_LEN: usize = 12;
const LATENT_AXIS_SCORE_FLOOR: f32 = 0.18;
const LATENT_AXIS_SPREAD_CEILING: f32 = 0.45;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaProbeFamily {
    pub name: String,
    pub model_id: String,
    pub output_contract: String,
    pub capability: String,
    pub output_len: usize,
    pub beta_core: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaProbeFamilyRun {
    pub family: GammaProbeFamily,
    pub run: AlphaProbeRun,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaProbeSuite {
    pub families: Vec<GammaProbeFamilyRun>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaLatentPromptVariant {
    pub name: String,
    pub prompt_id: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaLatentAxisVariantRun {
    pub variant: GammaLatentPromptVariant,
    pub run: AlphaProbeRun,
    pub score: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GammaLatentAxisStability {
    Stable {
        dominant_pole: String,
    },
    Unstable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaLatentAxisSweep {
    pub axis: String,
    pub left_pole: String,
    pub right_pole: String,
    pub model_id: String,
    pub output_contract: String,
    pub variants: Vec<GammaLatentAxisVariantRun>,
    pub mean_score: f32,
    pub spread: f32,
    pub stability: GammaLatentAxisStability,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaLatentSweepSuite {
    pub axes: Vec<GammaLatentAxisSweep>,
}

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

struct GammaLatentAxisSpec {
    axis: &'static str,
    left_pole: &'static str,
    right_pole: &'static str,
}

pub fn run_gamma_probe_suite(
    cache: &mut AlphaProbeCache,
    artifact: &AlphaArtifact,
    core_embedding: &AlphaProbeRun,
    core_affect: &AlphaProbeRun,
) -> Result<GammaProbeSuite, GammaError> {
    let mut families = vec![
        GammaProbeFamilyRun {
            family: GammaProbeFamily {
                name: "visual-semantic".into(),
                model_id: "model.alpha.embedding.v1".into(),
                output_contract: "contract.payload.embedding".into(),
                capability: "capability.visual-embedding".into(),
                output_len: core_embedding.values.len(),
                beta_core: true,
            },
            run: core_embedding.clone(),
        },
        GammaProbeFamilyRun {
            family: GammaProbeFamily {
                name: "affect-emotion".into(),
                model_id: "model.alpha.affect.v1".into(),
                output_contract: "contract.payload.logits".into(),
                capability: "capability.affective-axis-response".into(),
                output_len: core_affect.values.len(),
                beta_core: true,
            },
            run: core_affect.clone(),
        },
    ];

    for family in extension_families() {
        let run = run_configured_probe(cache, artifact, family_recipe(&family));
        families.push(GammaProbeFamilyRun {
            family,
            run,
        });
    }

    if families.len() != 6 {
        return Err(GammaError::new("gamma probe suite must expose six families"));
    }

    Ok(GammaProbeSuite {
        families,
    })
}

pub fn run_gamma_latent_sweep_suite(
    cache: &mut AlphaProbeCache,
    artifact: &AlphaArtifact,
) -> Result<GammaLatentSweepSuite, GammaError> {
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
        return Err(GammaError::new("gamma latent sweep suite must expose six axes"));
    }

    Ok(GammaLatentSweepSuite {
        axes,
    })
}

pub fn run_gamma_probe_validity_suite(
    latent_sweeps: &GammaLatentSweepSuite,
) -> Result<GammaProbeValiditySuite, GammaError> {
    let axes = latent_sweeps
        .axes
        .iter()
        .map(|axis| {
            let failure_modes = vec![
                prompt_sensitivity_failure(axis),
                blocked_failure(
                    GammaFailureMode::ModelDisagreement,
                    "model disagreement is not yet observable for latent-axis sweeps because no comparable alternate latent-axis model has been added",
                    "add a comparable latent-axis ensemble observable before allowing model-agreement promotions",
                ),
                blocked_failure(
                    GammaFailureMode::EmbeddingNeighborhoodInstability,
                    "embedding-neighborhood instability is not yet observable because gamma does not yet expose neighborhood replay evidence for latent-axis sweeps",
                    "add a neighborhood replay observable before allowing neighborhood-stability promotions",
                ),
                blocked_failure(
                    GammaFailureMode::LabelCollision,
                    "label collision is not yet observable because gamma does not yet expose a collision matrix for latent-axis labels",
                    "add latent-axis label collision checks before allowing label-stability promotions",
                ),
                blocked_failure(
                    GammaFailureMode::DomainMismatch,
                    "domain mismatch is not yet observable because gamma does not yet expose domain-fit evidence for latent-axis probes",
                    "add probe-to-artifact domain-fit evidence before allowing domain-fit promotions",
                ),
            ];
            let high_confidence_eligible = failure_modes
                .iter()
                .all(|failure| failure.disposition == GammaFailureModeDisposition::Clear);

            GammaAxisValidityAssessment {
                axis: axis.axis.clone(),
                high_confidence_eligible,
                failure_modes,
            }
        })
        .collect::<Vec<_>>();

    if axes.len() != latent_sweeps.axes.len() {
        return Err(GammaError::new("gamma probe validity suite must evaluate every latent axis"));
    }

    Ok(GammaProbeValiditySuite {
        axes,
    })
}

fn extension_families() -> Vec<GammaProbeFamily> {
    vec![
        GammaProbeFamily {
            name: "aesthetic".into(),
            model_id: "model.gamma.aesthetic.v1".into(),
            output_contract: "contract.payload.gamma.aesthetic".into(),
            capability: "capability.aesthetic-axis-response".into(),
            output_len: 14,
            beta_core: false,
        },
        GammaProbeFamily {
            name: "material-scene".into(),
            model_id: "model.gamma.material-scene.v1".into(),
            output_contract: "contract.payload.gamma.material-scene".into(),
            capability: "capability.material-scene-response".into(),
            output_len: 14,
            beta_core: false,
        },
        GammaProbeFamily {
            name: "color-harmony-context".into(),
            model_id: "model.gamma.color-harmony.v1".into(),
            output_contract: "contract.payload.gamma.color-harmony".into(),
            capability: "capability.color-harmony-response".into(),
            output_len: 10,
            beta_core: false,
        },
        GammaProbeFamily {
            name: "compatibility-harmonization".into(),
            model_id: "model.gamma.compatibility.v1".into(),
            output_contract: "contract.payload.gamma.compatibility".into(),
            capability: "capability.compatibility-response".into(),
            output_len: 10,
            beta_core: false,
        },
    ]
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

fn family_recipe(family: &GammaProbeFamily) -> ProbeRecipe {
    let (probe_id, prompt_id, created) = match family.name.as_str() {
        "aesthetic" => {
            ("probe-gamma-aesthetic", "policy.gamma.aesthetic.v1", crate::UtcMinute(202605250108))
        }
        "material-scene" => (
            "probe-gamma-material-scene",
            "policy.gamma.material-scene.v1",
            crate::UtcMinute(202605250109),
        ),
        "color-harmony-context" => (
            "probe-gamma-color-harmony",
            "policy.gamma.color-harmony.v1",
            crate::UtcMinute(202605250110),
        ),
        "compatibility-harmonization" => (
            "probe-gamma-compatibility",
            "policy.gamma.compatibility.v1",
            crate::UtcMinute(202605250111),
        ),
        _ => unreachable!("unsupported gamma probe family"),
    };

    ProbeRecipe {
        probe_id: probe_id.into(),
        name: family.name.clone(),
        model_id: family.model_id.clone(),
        prompt_id: prompt_id.into(),
        output_contract: family.output_contract.clone(),
        capability: family.capability.clone(),
        output_len: family.output_len,
        created,
    }
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

fn prompt_sensitivity_failure(axis: &GammaLatentAxisSweep) -> GammaFailureModeAssessment {
    match axis.stability {
        GammaLatentAxisStability::Stable {
            ..
        } => GammaFailureModeAssessment {
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
                "re-observe {} after adding the remaining G3 failure-mode observables",
                axis.axis
            )),
        },
    }
}

fn blocked_failure(
    mode: GammaFailureMode,
    detail: &str,
    required_follow_up: &str,
) -> GammaFailureModeAssessment {
    GammaFailureModeAssessment {
        mode,
        disposition: GammaFailureModeDisposition::Blocked,
        detail: detail.into(),
        required_follow_up: Some(required_follow_up.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alpha::{load_text, AlphaProbeCache};

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
