use serde::Deserialize;

use resonance::{
    load_public_fixtures, run_beta_text, run_gamma_text, run_gamma_text_with_config,
    AlphaProbeCache, GammaConfig, GammaFailureMode, GammaFailureModeDisposition,
    GammaLatentAxisStability, GammaPriorAlignment, GammaPriorSource,
};

const G2_LATENT_SWEEP_FIXTURE_JSON: &str =
    include_str!("../artifacts/gamma/g2-latent-axis-sweeps-v1.json");

#[derive(Debug, Deserialize)]
struct GammaLatentSweepFixture {
    source: String,
    text: String,
    extensions_disabled: bool,
    axes: Vec<GammaLatentAxisFixture>,
}

#[derive(Debug, Deserialize)]
struct GammaLatentAxisFixture {
    axis: String,
    left_pole: String,
    right_pole: String,
    model_id: String,
    output_contract: String,
    mean_score: f32,
    spread: f32,
    stability: String,
    dominant_pole: Option<String>,
    variants: Vec<GammaLatentVariantFixture>,
}

#[derive(Debug, Deserialize)]
struct GammaLatentVariantFixture {
    name: String,
    prompt_id: String,
    score: f32,
}

#[test]
fn gamma_reduces_to_beta_when_extensions_disabled() {
    let mut beta_cache = AlphaProbeCache::default();
    let beta = run_beta_text(
        &mut beta_cache,
        "artifact://gamma/text/demo",
        "Warm materials soften a bright scene while a tense hum stays underneath.",
    )
    .expect("beta run should succeed");
    let mut gamma_cache = AlphaProbeCache::default();
    let gamma = run_gamma_text(
        &mut gamma_cache,
        "artifact://gamma/text/demo",
        "Warm materials soften a bright scene while a tense hum stays underneath.",
    )
    .expect("gamma run should succeed");

    assert!(gamma.config.extensions_disabled);
    assert_eq!(gamma.probe_suite.families.len(), 6);
    assert_eq!(gamma.probe_suite.families[0].family.name, "visual-semantic");
    assert_eq!(gamma.probe_suite.families[1].family.name, "affect-emotion");
    assert!(gamma.probe_suite.families.iter().all(|family| !family.family.model_id.is_empty()));
    assert!(gamma
        .probe_suite
        .families
        .iter()
        .all(|family| !family.family.output_contract.is_empty()));
    assert!(gamma.latent_sweeps.extensions_disabled);
    assert!(gamma.latent_sweeps.axes.is_empty());
    assert!(gamma.probe_validity.extensions_disabled);
    assert!(gamma.probe_validity.axes.is_empty());
    assert!(gamma.prior_ensemble.extensions_disabled);
    assert!(gamma.prior_ensemble.priors.is_empty());
    assert_eq!(gamma.beta.gain.vector, beta.gain.vector);
    assert_eq!(gamma.beta.walk.state_after, beta.walk.state_after);
    assert_eq!(gamma.beta.disagreement.probe_disagreement, beta.disagreement.probe_disagreement,);
    assert_eq!(
        gamma.beta.disagreement.receptor_projection_disagreement,
        beta.disagreement.receptor_projection_disagreement,
    );
    assert_eq!(gamma.beta.report.snap_text, beta.report.snap_text);
}

#[test]
fn gamma_prior_ensemble_marks_each_prior_aligned_or_blocked() {
    let fixtures = load_public_fixtures().expect("beta public fixtures should load");
    let mut gamma_cache = AlphaProbeCache::default();
    let gamma = run_gamma_text_with_config(
        &mut gamma_cache,
        "artifact://gamma/text/prior-ensemble",
        "A bright mechanical corridor and warm reflective accents hold the scene in tension.",
        GammaConfig {
            extensions_disabled: false,
        },
    )
    .expect("gamma run should succeed");

    assert!(!gamma.prior_ensemble.extensions_disabled);

    let aligned = gamma
        .prior_ensemble
        .priors
        .iter()
        .filter(|prior| prior.alignment == GammaPriorAlignment::CoordinateAligned)
        .collect::<Vec<_>>();
    assert_eq!(aligned.len(), fixtures.priors.len());
    assert!(aligned.iter().all(|prior| prior.source == GammaPriorSource::ReceptorMaps));
    assert!(aligned
        .iter()
        .all(|prior| prior.atlas_id.as_deref() == Some(fixtures.atlas.id.as_str())));
    assert!(aligned.iter().all(|prior| prior.transform_id.is_some()));

    for fixture_prior in &fixtures.priors {
        assert!(aligned.iter().any(|prior| prior.prior_id == fixture_prior.id));
    }

    let blocked = gamma
        .prior_ensemble
        .priors
        .iter()
        .filter(|prior| prior.alignment == GammaPriorAlignment::Blocked)
        .collect::<Vec<_>>();
    assert_eq!(blocked.len(), 4);

    for source in [
        GammaPriorSource::FunctionalGradients,
        GammaPriorSource::StructuralConnectivity,
        GammaPriorSource::VisualBenchmarkPriors,
        GammaPriorSource::ImageryPriors,
    ] {
        let blocked_prior = blocked
            .iter()
            .find(|prior| prior.source == source)
            .expect("blocked prior source should exist");
        assert!(blocked_prior.required_follow_up.is_some());
        assert!(blocked_prior.atlas_id.is_none());
        assert!(blocked_prior.transform_id.is_none());
    }
}

#[test]
fn gamma_latent_axis_sweeps_mark_stability_per_axis() {
    let fixture = serde_json::from_str::<GammaLatentSweepFixture>(G2_LATENT_SWEEP_FIXTURE_JSON)
        .expect("gamma G2 latent sweep fixture should parse");
    let mut gamma_cache = AlphaProbeCache::default();
    let gamma = run_gamma_text_with_config(
        &mut gamma_cache,
        &fixture.source,
        &fixture.text,
        GammaConfig {
            extensions_disabled: false,
        },
    )
    .expect("gamma run should succeed");

    assert!(!gamma.config.extensions_disabled);
    assert_eq!(gamma.latent_sweeps.extensions_disabled, fixture.extensions_disabled);
    assert!(!gamma.latent_sweeps.extensions_disabled);
    assert_eq!(gamma.latent_sweeps.axes.len(), fixture.axes.len());

    for (axis, expected_axis) in gamma.latent_sweeps.axes.iter().zip(fixture.axes.iter()) {
        assert_eq!(axis.axis, expected_axis.axis);
        assert_eq!(axis.left_pole, expected_axis.left_pole);
        assert_eq!(axis.right_pole, expected_axis.right_pole);
        assert_eq!(axis.model_id, expected_axis.model_id);
        assert_eq!(axis.output_contract, expected_axis.output_contract);
        assert_eq!(axis.variants.len(), 3);
        assert_eq!(axis.variants.len(), expected_axis.variants.len());
        assert!(!axis.model_id.is_empty());
        assert!(!axis.output_contract.is_empty());
        assert!(axis.variants.iter().all(|variant| !variant.variant.prompt_id.is_empty()));

        for (variant, expected_variant) in axis.variants.iter().zip(expected_axis.variants.iter()) {
            assert_eq!(variant.variant.name, expected_variant.name);
            assert_eq!(variant.variant.prompt_id, expected_variant.prompt_id);
            assert_close(variant.score, expected_variant.score, &variant.variant.prompt_id);
        }

        let min_score =
            axis.variants.iter().map(|variant| variant.score).fold(f32::INFINITY, f32::min);
        let max_score =
            axis.variants.iter().map(|variant| variant.score).fold(f32::NEG_INFINITY, f32::max);
        let mean_score = axis.variants.iter().map(|variant| variant.score).sum::<f32>()
            / axis.variants.len() as f32;

        assert!((axis.spread - (max_score - min_score)).abs() <= f32::EPSILON);
        assert!((axis.mean_score - mean_score).abs() <= f32::EPSILON);
        assert_close(axis.mean_score, expected_axis.mean_score, &format!("{} mean", axis.axis));
        assert_close(axis.spread, expected_axis.spread, &format!("{} spread", axis.axis));

        match &axis.stability {
            GammaLatentAxisStability::Stable {
                dominant_pole,
            } => {
                assert_eq!(expected_axis.stability, "Stable");
                assert_eq!(expected_axis.dominant_pole.as_deref(), Some(dominant_pole.as_str()));
                assert!(dominant_pole == &axis.left_pole || dominant_pole == &axis.right_pole);
                if dominant_pole == &axis.left_pole {
                    assert!(axis.variants.iter().all(|variant| variant.score <= -0.18));
                } else {
                    assert!(axis.variants.iter().all(|variant| variant.score >= 0.18));
                }
                assert!(axis.spread <= 0.45);
            }
            GammaLatentAxisStability::Unstable => {
                assert_eq!(expected_axis.stability, "Unstable");
                assert_eq!(expected_axis.dominant_pole, None);
                let all_left = axis.variants.iter().all(|variant| variant.score <= -0.18);
                let all_right = axis.variants.iter().all(|variant| variant.score >= 0.18);
                assert!(!(axis.spread <= 0.45 && (all_left || all_right)));
            }
        }
    }
}

#[test]
fn gamma_probe_validity_blocks_unstable_axis_promotion() {
    let fixture = serde_json::from_str::<GammaLatentSweepFixture>(G2_LATENT_SWEEP_FIXTURE_JSON)
        .expect("gamma G2 latent sweep fixture should parse");
    let mut gamma_cache = AlphaProbeCache::default();
    let gamma = run_gamma_text_with_config(
        &mut gamma_cache,
        &fixture.source,
        &fixture.text,
        GammaConfig {
            extensions_disabled: false,
        },
    )
    .expect("gamma run should succeed");

    assert!(!gamma.probe_validity.extensions_disabled);
    assert_eq!(gamma.probe_validity.axes.len(), fixture.axes.len());

    for (assessment, expected_axis) in gamma.probe_validity.axes.iter().zip(fixture.axes.iter()) {
        assert_eq!(assessment.axis, expected_axis.axis);
        assert!(!assessment.high_confidence_eligible);
        assert_eq!(assessment.failure_modes.len(), 5);

        let prompt_sensitivity = assessment
            .failure_modes
            .iter()
            .find(|failure| failure.mode == GammaFailureMode::PromptSensitivity)
            .expect("prompt sensitivity assessment should exist");
        assert_eq!(prompt_sensitivity.disposition, GammaFailureModeDisposition::Observed);
        assert!(prompt_sensitivity.detail.contains("unstable"));
        assert!(prompt_sensitivity.required_follow_up.is_some());

        for kind in [
            GammaFailureMode::ModelDisagreement,
            GammaFailureMode::EmbeddingNeighborhoodInstability,
            GammaFailureMode::LabelCollision,
            GammaFailureMode::DomainMismatch,
        ] {
            let blocked = assessment
                .failure_modes
                .iter()
                .find(|failure| failure.mode == kind)
                .expect("blocked failure-mode assessment should exist");
            assert_eq!(blocked.disposition, GammaFailureModeDisposition::Blocked);
            assert!(blocked.required_follow_up.is_some());
        }
    }
}

fn assert_close(actual: f32, expected: f32, label: &str) {
    let delta = (actual - expected).abs();
    assert!(delta <= 1.0e-6, "{label} drifted: expected {expected:+.9}, got {actual:+.9}");
}
