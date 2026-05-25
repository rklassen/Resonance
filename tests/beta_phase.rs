use resonance::{run_beta_text, AlphaProbeCache, GateDecision};

#[test]
fn beta_public_priors_drive_a_replayable_runtime() {
    let mut first_cache = AlphaProbeCache::default();
    let first = run_beta_text(
        &mut first_cache,
        "artifact://beta/text/demo",
        "A cold reflective corridor gives way to a bright mechanical hum.",
    )
    .expect("beta text run should succeed");
    let mut second_cache = AlphaProbeCache::default();
    let second = run_beta_text(
        &mut second_cache,
        "artifact://beta/text/demo",
        "A cold reflective corridor gives way to a bright mechanical hum.",
    )
    .expect("beta replay should succeed");

    assert_eq!(first.gain.mapping_id, "real-prior-domain-bridge.beta.v2");
    assert_eq!(first.gain.terms.len(), 4);
    assert_eq!(first.gain.terms[0].family, "serotonin");
    assert_eq!(first.gain.terms[0].target, "5-HTT");
    assert_eq!(first.graph.node_count, 360);
    assert!(first.graph.non_zero_edges > 0);
    assert_eq!(first.report.gate_result.decision, GateDecision::Pass);
    assert!(first.report.claim.blocker.is_none());
    assert_eq!(first.disagreement.prior_ids.len(), 4);
    assert_eq!(first.gain.terms, second.gain.terms);
    assert_eq!(first.gain.vector, second.gain.vector);
    assert_eq!(first.walk.state_after, second.walk.state_after);
    assert_eq!(first.report.snap_text, second.report.snap_text);
}
