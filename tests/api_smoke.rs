use resonance::*;

#[test]
fn api_surface_smoke() {
    let artifact = ArtifactRecord {
        id: ArtifactId("artifact-1".into()),
        hash: HashDigest {
            algorithm: "sha256".into(),
            digest_hex: "abc123".into(),
        },
        source: ValueRef("file://artifacts/alpha/input.png".into()),
        metadata: vec![MetadataEntry {
            key: "media_type".into(),
            value: "image/png".into(),
        }],
    };

    let operator = OperatorDeclaration {
        id: OperatorId("operator-project-state".into()),
        name: "project-state".into(),
        inputs: vec![ContractId("contract.payload.embedding".into())],
        outputs: vec![ContractId("contract.payload.state".into())],
        capabilities: vec![CapabilityId("capability.state-projection".into())],
        runtime: RuntimePolicyId("runtime.detached".into()),
        determinism: DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken("Α".into())),
    };

    let operator_execution = OperatorExecutionRecord {
        id: ExecutionId("execution-operator-1".into()),
        operator: operator.id.clone(),
        input_artifacts: vec![artifact.id.clone()],
        input_payloads: vec![PayloadId("payload-embedding".into())],
        output_payloads: vec![PayloadId("payload-state".into())],
        output_gate_results: vec![GateResultId("gate-result-1".into())],
        output_traces: vec![TraceId("trace-1".into())],
        runtime: operator.runtime.clone(),
        created: UtcMinute(202605240000),
    };

    let payload = PayloadRecord {
        id: PayloadId("payload-state".into()),
        contract: ContractId("contract.payload.state".into()),
        producer: operator_execution.id.clone(),
        source_artifacts: vec![artifact.id.clone()],
        source_payloads: vec![PayloadId("payload-embedding".into())],
        value: ValueRef("zarr://cache/payloads/state.zarr/group/state".into()),
        hash: Some(HashDigest {
            algorithm: "sha256".into(),
            digest_hex: "def456".into(),
        }),
        numeric: Some(NumericPolicyId("numeric.signed-vector".into())),
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken("Α".into())),
        created: UtcMinute(202605240001),
    };

    let probe = ProbeDeclaration {
        id: ProbeId("probe-vision-embedding".into()),
        name: "vision-embedding".into(),
        inputs: vec![ContractId("contract.artifact.image".into())],
        outputs: vec![ContractId("contract.payload.embedding".into())],
        capabilities: vec![CapabilityId("capability.visual-embedding".into())],
        model: Some(ModelRef {
            id: "model-vision-1".into(),
            hash: HashDigest {
                algorithm: "sha256".into(),
                digest_hex: "model123".into(),
            },
            source: Some("registry://models/vision-embedding".into()),
            license: Some("license://apache-2.0".into()),
        }),
        prompt: Some(PolicyId("policy.zero-shot-labels".into())),
        preprocessing: PolicyId("policy.image-normalize".into()),
        runtime: RuntimePolicyId("runtime.detached".into()),
        tolerance: TolerancePolicyId("tolerance.cosine-1e-4".into()),
        overfit: OverfitPolicyId("overfit.frozen-baseline".into()),
        phase: Some(PhaseToken("Α".into())),
    };

    let probe_execution = ProbeExecutionRecord {
        id: ExecutionId("execution-probe-1".into()),
        probe: probe.id.clone(),
        input_artifact: Some(artifact.id.clone()),
        input_payloads: Vec::new(),
        model_hash: probe.model.as_ref().map(|model| model.hash.clone()),
        prompt_hash: Some(HashDigest {
            algorithm: "sha256".into(),
            digest_hex: "prompt123".into(),
        }),
        preprocessing_hash: HashDigest {
            algorithm: "sha256".into(),
            digest_hex: "prep123".into(),
        },
        runtime: probe.runtime.clone(),
        tolerance: probe.tolerance.clone(),
        output_payloads: vec![PayloadId("payload-embedding".into())],
        created: UtcMinute(202605240002),
    };

    let state = StateRecord {
        id: StateId("state-1".into()),
        subject: SubjectRef("payload:payload-state".into()),
        label: StateLabel("shape-verified".into()),
        phase: Some(PhaseToken("Α".into())),
        entered_by: Some(operator.id.clone()),
        exited_by: None,
        allowed_mutations: vec![MutationVerb::Promote, MutationVerb::Defer],
        evidence_payloads: vec![payload.id.clone()],
        evidence_gate_results: vec![GateResultId("gate-result-1".into())],
        evidence_traces: vec![TraceId("trace-1".into())],
        created: UtcMinute(202605240003),
    };

    let requirement = RequirementRecord {
        id: RequirementId("requirement-1".into()),
        name: "Payloads remain explicit".into(),
        statement: "Payload values remain referenced through value_ref.".into(),
        subject_contracts: vec![ContractId("contract.payload".into())],
        phases: vec![PhaseToken("Α".into())],
        verification_gates: vec![GateId("gate-payload-contract".into())],
        parents: Vec::new(),
        dependencies: Vec::new(),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        status: RequirementStatus::Active,
        created: UtcMinute(202605240004),
    };

    let gate = GateDeclaration {
        gate_id: GateId("gate-payload-contract".into()),
        display_name: "verify-payload-contract".into(),
        subject_contract: ContractId("contract.payload".into()),
        prerequisite_gate_ids: Vec::new(),
        fitness_function_id: FitnessFunctionId("fitness.payload-contract-complete".into()),
        phase_scope: Some(PhaseToken("Α".into())),
        applies_to_requirement_ids: vec![requirement.id.clone()],
        truth_axes: vec![
            TruthAxisId("Integration".into()),
            TruthAxisId("Performance".into()),
            TruthAxisId("Accuracy".into()),
        ],
        failure_policy: FailurePolicyId("failure.stop-and-surface".into()),
    };

    let gate_result = GateResult {
        gate_result_id: GateResultId("gate-result-1".into()),
        gate_id: gate.gate_id.clone(),
        subject_ref: SubjectRef("payload:payload-state".into()),
        prerequisite_results: Vec::new(),
        axis_results: vec![TruthAxisResult {
            axis_id: TruthAxisId("Integration".into()),
            judgment: TruthAxisJudgment::Yes,
            numeric_value: Some(1.0),
            evidence_refs: vec!["trace:trace-1".into()],
        }],
        decision: GateDecision::Pass,
        follow_up_observation: None,
        evidence_payload_ids: vec![payload.id.clone()],
        evidence_trace_ids: vec![TraceId("trace-1".into())],
        created: UtcMinute(202605240005),
    };

    let claim = ClaimRecord {
        id: ClaimRecordId("claim-1".into()),
        statement: "The payload remains trace-linked.".into(),
        status: ClaimStatus::ObservedFact,
        phase_scope: Some(PhaseToken("Α".into())),
        support_artifacts: vec![artifact.id.clone()],
        support_payloads: vec![payload.id.clone()],
        support_traces: vec![TraceId("trace-1".into())],
        support_gate_results: vec![gate_result.gate_result_id.clone()],
        support_snaps: vec![SnapRef("snap://trace/trace-1#node=a109".into())],
        uncertainty: Some(UncertaintyRecord {
            belief: Some(1.0),
            plausibility: Some(1.0),
            confidence: Some(1.0),
            conflict: Some(0.0),
            unsupported_mass: Some(0.0),
        }),
        blocker: None,
        created: UtcMinute(202605240006),
    };

    let trace = TraceRecord {
        id: TraceId("trace-1".into()),
        run: RunId("run-1".into()),
        phase: Some(PhaseToken("Α".into())),
        source_artifacts: vec![artifact.id.clone()],
        operator_executions: vec![operator_execution.id.clone(), probe_execution.id.clone()],
        payloads: vec![payload.id.clone(), PayloadId("payload-embedding".into())],
        path: SnapPathRef("snap://trace/trace-1".into()),
        gate_results: vec![gate_result.gate_result_id.clone()],
        claims: vec![claim.id.clone()],
        blocked_claims: vec![BlockedClaimId("blocked-claim-1".into())],
        replay: ReplayPolicyId("replay.canonical".into()),
        created: UtcMinute(202605240007),
    };

    let trace_step = TraceStep {
        id: TraceStepId("trace-step-1".into()),
        trace: trace.id.clone(),
        operator: operator.id.clone(),
        input_payloads: vec![PayloadId("payload-embedding".into())],
        output_payloads: vec![payload.id.clone()],
        snap_nodes: vec![SnapNodeRef("a105".into()), SnapNodeRef("a109".into())],
        snap_edges: vec![SnapEdgeRef("verify:a109->o109".into())],
        gate_results: vec![gate_result.gate_result_id.clone()],
        started: Some(UtcMinute(202605240008)),
        finished: Some(UtcMinute(202605240009)),
    };

    let output = OutputRecord {
        id: OutputId("output-1".into()),
        name: "alpha-shape-report".into(),
        source_traces: vec![trace.id.clone()],
        included_claims: vec![claim.id.clone()],
        included_gate_results: vec![gate_result.gate_result_id.clone()],
        export: ValueRef("file://output/reports/alpha-shape-report.md".into()),
        generator: operator.id.clone(),
        phase: Some(PhaseToken("Α".into())),
        created: UtcMinute(202605240010),
    };

    assert_eq!(artifact.metadata.len(), 1);
    assert_eq!(operator_execution.output_payloads.len(), 1);
    assert_eq!(probe_execution.output_payloads.len(), 1);
    assert_eq!(state.allowed_mutations.len(), 2);
    assert_eq!(requirement.status, RequirementStatus::Active);
    assert_eq!(gate_result.decision, GateDecision::Pass);
    assert_eq!(trace.claims, vec![claim.id.clone()]);
    assert_eq!(trace_step.trace, trace.id);
    assert_eq!(output.included_claims, vec![claim.id]);
}
