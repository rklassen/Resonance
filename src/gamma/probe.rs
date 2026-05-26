use crate::alpha::{
    run_configured_probe, AlphaArtifact, AlphaProbeCache, AlphaProbeRun, ProbeRecipe,
};

use crate::SemanticError;

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

pub fn run_gamma_probe_suite(
    cache: &mut AlphaProbeCache,
    artifact: &AlphaArtifact,
    core_embedding: &AlphaProbeRun,
    core_affect: &AlphaProbeRun,
) -> crate::SemanticResult<GammaProbeSuite> {
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
        return Err(SemanticError::new("gamma probe suite must expose six families"));
    }

    Ok(GammaProbeSuite {
        families,
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
