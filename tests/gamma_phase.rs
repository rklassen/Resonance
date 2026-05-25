use resonance::{run_beta_text, run_gamma_text, AlphaProbeCache};

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
    assert_eq!(gamma.beta.gain.vector, beta.gain.vector);
    assert_eq!(gamma.beta.walk.state_after, beta.walk.state_after);
    assert_eq!(gamma.beta.disagreement.probe_disagreement, beta.disagreement.probe_disagreement,);
    assert_eq!(
        gamma.beta.disagreement.receptor_projection_disagreement,
        beta.disagreement.receptor_projection_disagreement,
    );
    assert_eq!(gamma.beta.report.snap_text, beta.report.snap_text);
}
