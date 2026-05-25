use resonance::{
    laplacian_walk, load_text, mock_receptor_gain, project_to_vibes, run_alpha_image,
    run_alpha_text, run_embedding_probe, run_label_probe, AlphaParcelGraph, AlphaProbeCache,
    AlphaSnapDocument, CacheStatus, ClaimStatus, GateDecision, TruthAxisJudgment, VIBES_12D,
};

#[test]
fn alpha_observations_are_deterministic_in_release() {
    let mut cache = AlphaProbeCache::default();
    let first = run_alpha_text(
        &mut cache,
        "file://artifacts/alpha/specimen.txt",
        "Resonance\r\nalpha specimen\r\n",
    )
    .expect("first alpha run should succeed");
    let second = run_alpha_text(
        &mut cache,
        "file://artifacts/alpha/specimen.txt",
        "Resonance\r\nalpha specimen\r\n",
    )
    .expect("second alpha run should succeed");

    let reparsed = AlphaSnapDocument::parse(&first.report.snap_text)
        .expect("alpha snap should parse on roundtrip");

    assert_eq!(reparsed.to_text(), first.report.snap_text, "O1 snap roundtrip stable");
    assert_eq!(
        first.artifact.record.hash, second.artifact.record.hash,
        "artifact hash deterministic"
    );
    assert_eq!(
        first.embedding_probe.values, second.embedding_probe.values,
        "embedding probe deterministic"
    );
    assert_eq!(first.label_probe.values, second.label_probe.values, "label probe deterministic");
    assert_eq!(
        first.embedding_probe.cache_status,
        CacheStatus::Miss,
        "first embedding run should miss cache"
    );
    assert_eq!(
        second.embedding_probe.cache_status,
        CacheStatus::Hit,
        "second embedding run should hit cache"
    );
    assert_eq!(
        first.label_probe.cache_status,
        CacheStatus::Miss,
        "first label run should miss cache"
    );
    assert_eq!(
        second.label_probe.cache_status,
        CacheStatus::Hit,
        "second label run should hit cache"
    );
    assert_eq!(cache.len(), 2, "two deterministic cache entries should exist");

    assert!(first.vibes.validation.ranges_valid, "12D/11D vibes ranges stay signed and finite");
    assert!(first.vibes.validation.collapse_valid, "12D to 11D collapse is stable");
    assert!(first.vibes.validation.roles_valid, "vibes role metadata is valid");
    assert_eq!(first.vibes.signed_12d.len(), VIBES_12D.len(), "12D vibes shape stays fixed");
    assert_eq!(first.vibes.signed_11d.len(), 11, "11D vibes shape stays fixed");
    assert!(
        first
            .vibes
            .signed_12d
            .iter()
            .chain(first.vibes.signed_11d.iter())
            .all(|value| (-1.0..=1.0).contains(value)),
        "vibes values remain in signed range"
    );
    assert_eq!(first.graph.node_count, 360, "mock parcel graph keeps the alpha 360-node shape");
    assert!(
        first.graph.non_zero_edges < first.graph.node_count * 8,
        "mock parcel graph remains sparse in alpha"
    );
    assert_eq!(first.gain.vector.len(), 360, "mock receptor gain matches parcel graph width");
    assert_eq!(
        first.walk.state_after.len(),
        360,
        "laplacian walk output matches parcel graph width"
    );
    assert_eq!(
        first.report.trace.operator_executions.len(),
        9,
        "totality trace records intake, two probes, two cache uses, projection, gain, walk, and report"
    );
    assert_eq!(
        first.report.steps.len(),
        9,
        "trace steps cover every alpha module from intake through report"
    );
    assert_eq!(
        first.report.gate_declaration.truth_axes.len(),
        1,
        "alpha totality gate remains phase-appropriate and integration-scoped"
    );
    assert_eq!(
        first.report.gate_declaration.display_name, "verify-alpha-totality-integration",
        "alpha report declares the expected verification gate"
    );
    assert_eq!(
        first.report.gate_result.gate_id, first.report.gate_declaration.gate_id,
        "gate result stays attached to the declared gate"
    );
    assert_eq!(
        first.report.gate_result.subject_ref.0,
        format!("trace:{}", first.report.trace.id.0),
        "gate result directly observes the alpha trace subject"
    );
    assert_eq!(
        first.report.gate_result.axis_results.len(),
        1,
        "alpha totality gate reports exactly one integration axis result"
    );
    assert_eq!(
        first.report.gate_result.axis_results[0].axis_id.0, "Integration",
        "alpha totality gate stays integration-scoped"
    );
    assert_eq!(
        first.report.gate_result.axis_results[0].judgment,
        TruthAxisJudgment::Yes,
        "alpha totality gate passes with an affirmative judgment"
    );
    assert_eq!(
        first.report.gate_result.axis_results[0].numeric_value,
        Some(1.0),
        "alpha totality gate records complete integration evidence"
    );
    assert_eq!(
        first.report.gate_result.decision,
        GateDecision::Pass,
        "alpha totality gate must pass when all required steps are recorded"
    );
    assert!(
        first.report.gate_result.follow_up_observation.is_none(),
        "alpha totality gate should not emit a follow-up observation on pass"
    );
    assert_eq!(
        first.report.claim.status,
        ClaimStatus::DerivedClaim,
        "alpha report materializes a derived claim from the verified trace"
    );
    assert!(
        first
            .report
            .claim
            .statement
            .contains("Alpha totality trace covers intake, probes, cache, projection, mock gain, walk, and reporting"),
        "alpha claim directly states the verified totality observation"
    );
    assert_eq!(
        first.report.claim.support_traces,
        vec![first.report.trace.id.clone()],
        "alpha claim cites the verified trace directly"
    );
    assert_eq!(
        first.report.claim.support_gate_results,
        vec![first.report.gate_result.gate_result_id.clone()],
        "alpha claim cites the passing gate result directly"
    );
    assert_eq!(
        first.report.claim.support_snaps.iter().map(|snap| snap.0.as_str()).collect::<Vec<_>>(),
        vec![
            "snap://alpha/trace#node=o101",
            "snap://alpha/trace#node=o103",
            "snap://alpha/trace#node=o105",
            "snap://alpha/trace#node=o109",
        ],
        "alpha claim cites the four declared observation nodes directly"
    );
    let blocker = first
        .report
        .claim
        .blocker
        .as_ref()
        .expect("alpha claim should keep the phase-scoped mock blocker visible");
    assert_eq!(
        blocker.blocker.0, "alpha-mocks-phase-scoped",
        "alpha completion must keep its mock-phase caveat explicit"
    );
    assert_eq!(
        blocker.missing_dependency.as_deref(),
        Some("beta real priors and parcel graph"),
        "alpha blocker names the beta dependency explicitly"
    );
    assert_eq!(
        blocker.required_follow_up.as_deref(),
        Some("replace mocks in beta"),
        "alpha blocker keeps the required beta follow-up explicit"
    );
    assert_eq!(
        first.report.output.source_traces,
        vec![first.report.trace.id.clone()],
        "alpha output records the trace it exports"
    );
    assert_eq!(
        first.report.output.included_claims,
        vec![first.report.claim.id.clone()],
        "alpha output includes the verified claim directly"
    );
    assert_eq!(
        first.report.output.included_gate_results,
        vec![first.report.gate_result.gate_result_id.clone()],
        "alpha output includes the passing gate result directly"
    );
    assert_eq!(
        first.report.output.export.0,
        format!("file://output/reports/{}.snap", first.report.trace.id.0),
        "alpha output export path stays trace-addressed"
    );
    assert_eq!(
        first.report.output.generator.0, "operator-alpha-trace-report",
        "alpha output stays attached to the trace-report operator"
    );
    assert_eq!(
        first.report.output.phase.as_ref().map(|phase| phase.0.as_str()),
        Some("Α"),
        "alpha output remains phase-scoped"
    );
    assert!(
        first
            .report
            .snap
            .nodes
            .iter()
            .any(|node| node.id == "o101" && node.name == "Observe-Snap-Roundtrip❇alpha"),
        "alpha snap declares the roundtrip observation node"
    );
    assert!(
        first
            .report
            .snap
            .nodes
            .iter()
            .any(|node| node.id == "o103" && node.name == "Observe-Probe-Stability❇alpha"),
        "alpha snap declares the probe-stability observation node"
    );
    assert!(
        first
            .report
            .snap
            .nodes
            .iter()
            .any(|node| node.id == "o105" && node.name == "Observe-Vibes-Vector❇alpha"),
        "alpha snap declares the vibes-vector observation node"
    );
    assert!(
        first
            .report
            .snap
            .nodes
            .iter()
            .any(|node| node.id == "o109" && node.name == "Observe-Totality-Trace❇alpha"),
        "alpha snap declares the totality-trace observation node"
    );
    let verify_edges = first
        .report
        .snap
        .edge_groups
        .iter()
        .find(|group| group.family == "verify")
        .expect("alpha snap should declare verify edges");
    assert_eq!(
        verify_edges.edges,
        vec![
            ("a101".into(), "o101".into()),
            ("a103".into(), "o103".into()),
            ("a105".into(), "o105".into()),
            ("a109".into(), "o109".into()),
        ],
        "alpha snap verify edges keep the declared observation wiring"
    );

    assert_eq!(first.report.trace, second.report.trace, "trace exists and is deterministic");
    assert_eq!(first.report.snap_text, second.report.snap_text, "snap trace text is deterministic");
    assert_eq!(first.walk.state_after, second.walk.state_after, "toy walk is deterministic");
}

#[test]
fn alpha_image_intake_is_deterministic() {
    let mut cache = AlphaProbeCache::default();
    let bytes = vec![137, 80, 78, 71, 13, 10, 26, 10, 1, 2, 3, 4, 5, 6, 7, 8];
    let left =
        run_alpha_image(&mut cache, "file://artifacts/alpha/specimen.png", "image/png", &bytes)
            .expect("image alpha run should succeed");
    let right =
        run_alpha_image(&mut cache, "file://artifacts/alpha/specimen.png", "image/png", &bytes)
            .expect("repeat image alpha run should succeed");

    assert_eq!(left.artifact.record.hash, right.artifact.record.hash);
    assert_eq!(left.report.trace, right.report.trace);
}

#[test]
fn alpha_walk_rejects_gain_width_mismatch() {
    let mut cache = AlphaProbeCache::default();
    let artifact = load_text("file://artifacts/alpha/specimen.txt", "Resonance alpha specimen\n");
    let embedding = run_embedding_probe(&mut cache, &artifact);
    let labels = run_label_probe(&mut cache, &artifact);
    let vibes = project_to_vibes(&embedding, &labels).expect("vibes projection should succeed");
    let mut gain = mock_receptor_gain(&vibes).expect("mock gain should succeed");
    gain.vector.pop();

    let err = laplacian_walk(&AlphaParcelGraph::mock_360(), &vibes, &gain)
        .expect_err("walk should reject mismatched gain width");
    assert!(err.to_string().contains("gain width mismatch"));
}

#[test]
fn alpha_walk_rejects_delta_width_mismatch() {
    let mut cache = AlphaProbeCache::default();
    let artifact = load_text("file://artifacts/alpha/specimen.txt", "Resonance alpha specimen\n");
    let embedding = run_embedding_probe(&mut cache, &artifact);
    let labels = run_label_probe(&mut cache, &artifact);
    let vibes = project_to_vibes(&embedding, &labels).expect("vibes projection should succeed");
    let gain = mock_receptor_gain(&vibes).expect("mock gain should succeed");
    let graph = AlphaParcelGraph {
        node_count: 360,
        adjacency: Vec::new(),
        degree: Vec::new(),
        non_zero_edges: 0,
    };

    let err = laplacian_walk(&graph, &vibes, &gain)
        .expect_err("walk should reject mismatched laplacian width");
    assert!(err.to_string().contains("delta width mismatch"));
}
