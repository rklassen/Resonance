use serde::Deserialize;

use resonance::{
    load_public_fixtures, run_beta_text, run_gamma_text, AlphaProbeCache, GammaFailureMode,
    GammaFailureModeDisposition, GammaLatentAxisStability, GammaPriorAlignment, GammaPriorSource,
};

const G2_LATENT_SWEEP_FIXTURE_JSON: &str =
    include_str!("../artifacts/gamma/g2-latent-axis-sweeps-v1.json");

#[derive(Debug, Deserialize)]
struct GammaLatentSweepFixture {
    source: String,
    text: String,
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
fn gamma_preserves_beta_substrate_while_extending_gamma_surfaces() {
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

    assert_eq!(gamma.probe_suite.families.len(), 6);
    assert_eq!(gamma.probe_suite.families[0].family.name, "visual-semantic");
    assert_eq!(gamma.probe_suite.families[1].family.name, "affect-emotion");
    assert!(gamma.probe_suite.families.iter().all(|family| !family.family.model_id.is_empty()));
    assert!(gamma
        .probe_suite
        .families
        .iter()
        .all(|family| !family.family.output_contract.is_empty()));
    assert_eq!(gamma.latent_sweeps.axes.len(), 6);
    assert_eq!(gamma.probe_validity.axes.len(), gamma.latent_sweeps.axes.len());
    assert!(!gamma.prior_ensemble.priors.is_empty());
    assert!(!gamma.receptor_bridge.families.is_empty());
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
fn gamma_receptor_bridge_emits_provenanced_family_comparisons() {
    let mut gamma_cache = AlphaProbeCache::default();
    let gamma = run_gamma_text(
        &mut gamma_cache,
        "artifact://gamma/text/receptor-bridge",
        "A warm reflective corridor vibrates with bright motion and mechanical tension.",
    )
    .expect("gamma run should succeed");

    assert!(gamma.receptor_bridge.families.len() >= 3);

    for family in &gamma.receptor_bridge.families {
        assert!(!family.prior_ids.is_empty());
        assert!(!family.atlas_ids.is_empty());
        assert!(!family.transform_ids.is_empty());
        if family.unsupported_family {
            assert!(family.family.is_none());
            assert!(family.gain_mean.is_none());
            assert!(family.gain_variance.is_none());
            assert!(family.required_follow_up.is_some());
        } else {
            assert!(family.family.is_some());
            assert!(family.gain_mean.is_some());
            assert!(family.gain_variance.is_some());
            assert!(family.required_follow_up.is_none());
        }
    }

    assert!(gamma
        .receptor_bridge
        .families
        .iter()
        .any(|family| family.family.as_deref() == Some("serotonin")
            && family.prior_ids.len() == 2
            && family.source_disagreement.is_none()));
    assert!(gamma
        .receptor_bridge
        .families
        .iter()
        .any(|family| family.family.as_deref() == Some("glutamate")));
    assert!(gamma
        .receptor_bridge
        .families
        .iter()
        .any(|family| family.family.as_deref() == Some("norepinephrine")));
}

#[test]
fn gamma_dual_path_runtime_emits_independent_path_traces_before_comparison() {
    let mut gamma_cache = AlphaProbeCache::default();
    let gamma = run_gamma_text(
        &mut gamma_cache,
        "artifact://gamma/text/dual-path",
        "A reflective hallway holds a bright pulse while a tense narrative hum keeps rising.",
    )
    .expect("gamma run should succeed");

    let objective = &gamma.dual_path_runtime.objective_path;
    let narrative = &gamma.dual_path_runtime.narrative_path;

    assert_eq!(objective.state_seed.len(), gamma.beta.graph.node_count);
    assert!(objective.trace.operator_executions.contains(&objective.execution.id));
    assert!(objective.trace.operator_executions.contains(&objective.graph_execution.id));
    assert!(objective.trace.payloads.contains(&objective.graph_payload.id));
    assert!(objective.trace.payloads.contains(&objective.payload.id));
    assert!(objective.trace.payloads.contains(&gamma.beta.embedding_probe.payload.id));
    assert!(objective.trace.gate_results.is_empty());

    assert!(narrative.trace.payloads.contains(&gamma.beta.label_probe.payload.id));
    assert!(narrative.trace.payloads.contains(&gamma.beta.vibes.payload_12d.id));
    assert!(narrative.trace.payloads.contains(&gamma.beta.vibes.payload_11d.id));
    assert!(narrative.trace.payloads.contains(&gamma.beta.gain.payload.id));
    assert!(narrative.trace.payloads.contains(&narrative.bridge_payload.id));
    assert_eq!(narrative.bridge_execution.input_payloads, vec![gamma.beta.gain.payload.id.clone()]);
    assert_eq!(narrative.family_names.len(), narrative.family_mean_vector.len());
    assert!(!narrative.family_names.is_empty());

    assert_ne!(objective.trace.id, narrative.trace.id);
    assert!(narrative.trace.gate_results.is_empty());
}

#[test]
fn gamma_prior_ensemble_marks_each_prior_aligned_or_blocked() {
    let fixtures = load_public_fixtures().expect("beta public fixtures should load");
    let mut gamma_cache = AlphaProbeCache::default();
    let gamma = run_gamma_text(
        &mut gamma_cache,
        "artifact://gamma/text/prior-ensemble",
        "A bright mechanical corridor and warm reflective accents hold the scene in tension.",
    )
    .expect("gamma run should succeed");

    let aligned = gamma
        .prior_ensemble
        .priors
        .iter()
        .filter(|prior| prior.alignment == GammaPriorAlignment::CoordinateAligned)
        .collect::<Vec<_>>();
    assert_eq!(aligned.len(), fixtures.priors.len());
    assert!(aligned.iter().all(|prior| prior.source == GammaPriorSource::ReceptorMaps));
    assert!(aligned.iter().all(|prior| prior.source_record.is_some()));
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
    let gamma = run_gamma_text(&mut gamma_cache, &fixture.source, &fixture.text)
        .expect("gamma run should succeed");

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
    let gamma = run_gamma_text(&mut gamma_cache, &fixture.source, &fixture.text)
        .expect("gamma run should succeed");

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
