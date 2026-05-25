use crate::{
    beta::BetaRun, CapabilityId, ContractId, DeterminismPolicyId, ExecutionId, FailurePolicyId,
    FitnessFunctionId, GateDecision, GateDeclaration, GateId, GateResult, GateResultId,
    NumericPolicyId, OperatorDeclaration, OperatorExecutionRecord, OperatorId, PayloadId,
    PayloadRecord, PhaseToken, ProvenancePolicyId, ReplayPolicyId, RequirementId, RunId,
    RuntimePolicyId, SideEffectPolicyId, SnapEdgeRef, SnapNodeRef, SnapPathRef, SubjectRef,
    TraceId, TraceRecord, TruthAxisId, TruthAxisJudgment, TruthAxisResult, UtcMinute, ValueRef,
};

use super::{
    canonical_vector, probe_trace_step, sha256_hex, short_id, trace_step, GammaError,
    GammaNarrativeParcelFeedback, GammaNarrativePath, GAMMA_PHASE,
};

pub(super) fn build_narrative_path(beta: &BetaRun) -> Result<GammaNarrativePath, GammaError> {
    let supported_families = beta.gain.terms.iter().fold(
        std::collections::BTreeMap::<String, Vec<f32>>::new(),
        |mut grouped, term| {
            grouped.entry(term.family.clone()).or_default().push(term.coefficient);
            grouped
        },
    );
    let family_names = supported_families.keys().cloned().collect::<Vec<_>>();
    let family_mean_vector = supported_families
        .values()
        .map(|coefficients| coefficients.iter().sum::<f32>() / coefficients.len() as f32)
        .collect::<Vec<_>>();
    if family_names.is_empty() {
        return Err(GammaError::new(
            "gamma narrative path must emit at least one receptor family summary",
        ));
    }

    let supported_count = family_names.len();
    let digest = sha256_hex(&[
        canonical_vector(&family_mean_vector).as_bytes(),
        family_names.join("|").as_bytes(),
        beta.gain.payload.id.0.as_bytes(),
    ]);

    let bridge_declaration = OperatorDeclaration {
        id: OperatorId("operator-gamma-narrative-bridge".into()),
        name: "gamma-narrative-receptor-bridge".into(),
        inputs: vec![beta.gain.payload.contract.clone()],
        outputs: vec![ContractId("contract.payload.gamma.narrative-receptor-families".into())],
        capabilities: vec![CapabilityId("capability.gamma-narrative-bridge".into())],
        runtime: RuntimePolicyId("runtime.gamma.replayable".into()),
        determinism: DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
    };
    let bridge_execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-gamma-narrative-bridge-{}", short_id(&digest))),
        operator: bridge_declaration.id.clone(),
        input_artifacts: vec![beta.artifact.record.id.clone()],
        input_payloads: vec![beta.gain.payload.id.clone()],
        output_payloads: vec![PayloadId(format!(
            "payload-gamma-narrative-bridge-{}",
            short_id(&digest)
        ))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: bridge_declaration.runtime.clone(),
        created: UtcMinute(202605250203),
    };
    let bridge_payload = PayloadRecord {
        id: bridge_execution.output_payloads[0].clone(),
        contract: ContractId("contract.payload.gamma.narrative-receptor-families".into()),
        producer: bridge_execution.id.clone(),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        source_payloads: vec![beta.gain.payload.id.clone()],
        value: ValueRef("inline://gamma/narrative/receptor-families".into()),
        hash: Some(crate::HashDigest {
            algorithm: "sha256".into(),
            digest_hex: digest,
        }),
        numeric: Some(NumericPolicyId("numeric.signed-vector".into())),
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250203),
    };
    let parcel_feedback_vector = build_parcel_feedback_vector(&beta.gain.vector, &family_mean_vector);
    let parcel_feedback_digest = sha256_hex(&[
        canonical_vector(&parcel_feedback_vector).as_bytes(),
        bridge_payload.id.0.as_bytes(),
        beta.gain.payload.id.0.as_bytes(),
    ]);
    let parcel_feedback_declaration = OperatorDeclaration {
        id: OperatorId("operator-gamma-narrative-parcel-feedback".into()),
        name: "gamma-narrative-parcel-feedback".into(),
        inputs: vec![bridge_payload.contract.clone(), beta.gain.payload.contract.clone()],
        outputs: vec![ContractId("contract.payload.gamma.narrative-parcel-feedback-360".into())],
        capabilities: vec![CapabilityId("capability.gamma-narrative-parcel-feedback".into())],
        runtime: RuntimePolicyId("runtime.gamma.replayable".into()),
        determinism: DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
    };
    let parcel_feedback_execution = OperatorExecutionRecord {
        id: ExecutionId(format!(
            "execution-gamma-narrative-parcel-feedback-{}",
            short_id(&parcel_feedback_digest)
        )),
        operator: parcel_feedback_declaration.id.clone(),
        input_artifacts: vec![beta.artifact.record.id.clone()],
        input_payloads: vec![bridge_payload.id.clone(), beta.gain.payload.id.clone()],
        output_payloads: vec![PayloadId(format!(
            "payload-gamma-narrative-parcel-feedback-{}",
            short_id(&parcel_feedback_digest)
        ))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: parcel_feedback_declaration.runtime.clone(),
        created: UtcMinute(202605250204),
    };
    let parcel_feedback_payload = PayloadRecord {
        id: parcel_feedback_execution.output_payloads[0].clone(),
        contract: ContractId("contract.payload.gamma.narrative-parcel-feedback-360".into()),
        producer: parcel_feedback_execution.id.clone(),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        source_payloads: vec![bridge_payload.id.clone(), beta.gain.payload.id.clone()],
        value: ValueRef("inline://gamma/narrative/parcel-feedback-360".into()),
        hash: Some(crate::HashDigest {
            algorithm: "sha256".into(),
            digest_hex: parcel_feedback_digest,
        }),
        numeric: Some(NumericPolicyId("numeric.signed-vector.360".into())),
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250204),
    };
    let parcel_feedback_valid = parcel_feedback_vector.len() == beta.graph.node_count
        && parcel_feedback_vector.iter().all(|value| value.is_finite());
    let parcel_feedback_gate_declaration = GateDeclaration {
        gate_id: GateId("gate-gamma-narrative-parcel-feedback".into()),
        display_name: "verify-gamma-narrative-parcel-feedback".into(),
        subject_contract: parcel_feedback_payload.contract.clone(),
        prerequisite_gate_ids: Vec::new(),
        fitness_function_id: FitnessFunctionId("fitness.gamma-narrative-parcel-feedback".into()),
        phase_scope: Some(PhaseToken(GAMMA_PHASE.into())),
        applies_to_requirement_ids: vec![RequirementId(
            "requirement.gamma.narrative-parcel-feedback".into(),
        )],
        truth_axes: vec![TruthAxisId("Integration".into())],
        failure_policy: FailurePolicyId("failure.stop-and-surface".into()),
    };
    let parcel_feedback_gate_result = GateResult {
        gate_result_id: GateResultId("gate-result-gamma-narrative-parcel-feedback".into()),
        gate_id: parcel_feedback_gate_declaration.gate_id.clone(),
        subject_ref: SubjectRef(format!("payload:{}", parcel_feedback_payload.id.0)),
        prerequisite_results: Vec::new(),
        axis_results: vec![TruthAxisResult {
            axis_id: TruthAxisId("Integration".into()),
            judgment: if parcel_feedback_valid {
                TruthAxisJudgment::Yes
            } else {
                TruthAxisJudgment::Weak
            },
            numeric_value: Some(if parcel_feedback_valid { 1.0 } else { 0.25 }),
            evidence_refs: vec![
                "observation:parcel-feedback-width-matches-graph".into(),
                "observation:parcel-feedback-values-are-finite".into(),
            ],
        }],
        decision: if parcel_feedback_valid {
            GateDecision::Pass
        } else {
            GateDecision::Fail
        },
        follow_up_observation: None,
        evidence_payload_ids: vec![
            bridge_payload.id.clone(),
            beta.gain.payload.id.clone(),
            parcel_feedback_payload.id.clone(),
        ],
        evidence_trace_ids: vec![TraceId(format!(
            "trace-gamma-narrative-{}",
            beta.artifact.record.id.0,
        ))],
        created: UtcMinute(202605250204),
    };
    let trace = TraceRecord {
        id: TraceId(format!("trace-gamma-narrative-{}", beta.artifact.record.id.0)),
        run: RunId(format!("run-gamma-{}", beta.artifact.record.id.0)),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        operator_executions: vec![
            beta.artifact.execution.id.clone(),
            beta.label_probe.execution.id.clone(),
            beta.label_probe.cache_execution.id.clone(),
            beta.vibes.execution.id.clone(),
            beta.gain.execution.id.clone(),
            bridge_execution.id.clone(),
            parcel_feedback_execution.id.clone(),
        ],
        payloads: vec![
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.vibes.payload_11d.id.clone(),
            beta.gain.payload.id.clone(),
            bridge_payload.id.clone(),
            parcel_feedback_payload.id.clone(),
        ],
        path: SnapPathRef(format!("snap://gamma/narrative/{}", beta.artifact.record.id.0)),
        gate_results: vec![parcel_feedback_gate_result.gate_result_id.clone()],
        claims: Vec::new(),
        blocked_claims: Vec::new(),
        replay: ReplayPolicyId("replay.canonical".into()),
        created: UtcMinute(202605250203),
    };
    let steps = vec![
        trace_step(
            "trace-step-gamma-narrative-intake",
            &trace.id,
            &beta.artifact.execution,
            vec![SnapNodeRef("g611".into())],
            vec![SnapEdgeRef("flow:g610->g611".into())],
        ),
        probe_trace_step(
            "trace-step-gamma-narrative-label",
            &trace.id,
            &beta.label_probe.execution,
            vec![SnapNodeRef("g612".into())],
            vec![SnapEdgeRef("flow:g611->g612".into())],
        ),
        trace_step(
            "trace-step-gamma-narrative-cache",
            &trace.id,
            &beta.label_probe.cache_execution,
            vec![SnapNodeRef("g613".into())],
            vec![SnapEdgeRef("flow:g612->g613".into())],
        ),
        trace_step(
            "trace-step-gamma-narrative-vibes",
            &trace.id,
            &beta.vibes.execution,
            vec![SnapNodeRef("g614".into())],
            vec![SnapEdgeRef("flow:g613->g614".into())],
        ),
        trace_step(
            "trace-step-gamma-narrative-gain",
            &trace.id,
            &beta.gain.execution,
            vec![SnapNodeRef("g615".into())],
            vec![SnapEdgeRef("flow:g614->g615".into())],
        ),
        trace_step(
            "trace-step-gamma-narrative-bridge",
            &trace.id,
            &bridge_execution,
            vec![SnapNodeRef("g616".into())],
            vec![SnapEdgeRef("flow:g615->g616".into())],
        ),
        trace_step(
            "trace-step-gamma-narrative-parcel-feedback",
            &trace.id,
            &parcel_feedback_execution,
            vec![SnapNodeRef("g616".into()), SnapNodeRef("g617".into())],
            vec![SnapEdgeRef("flow:g616->g617".into())],
        ),
    ];

    Ok(GammaNarrativePath {
        bridge_declaration,
        bridge_execution,
        bridge_payload,
        parcel_feedback: GammaNarrativeParcelFeedback {
            declaration: parcel_feedback_declaration,
            execution: parcel_feedback_execution,
            payload: parcel_feedback_payload,
            gate_declaration: parcel_feedback_gate_declaration,
            gate_result: parcel_feedback_gate_result,
            vector: parcel_feedback_vector,
            detail: "narrative parcel feedback scales the beta receptor-gain substrate by the traced family summary so recirculation stays parcel-typed and provenance-backed".into(),
        },
        family_names,
        family_mean_vector,
        trace,
        steps,
        detail: format!(
            "narrative path traces the semantic and affect branch through the shared beta vibes and gain substrate into {} supported receptor family summary value(s)",
            supported_count,
        ),
    })
}

fn build_parcel_feedback_vector(gain_vector: &[f32], family_mean_vector: &[f32]) -> Vec<f32> {
    let signed_mean = family_mean_vector.iter().sum::<f32>() / family_mean_vector.len() as f32;
    let mean_magnitude = family_mean_vector.iter().map(|value| value.abs()).sum::<f32>()
        / family_mean_vector.len() as f32;

    gain_vector
        .iter()
        .map(|gain_value| ((signed_mean * 0.5) + (mean_magnitude * 0.5 * gain_value)).clamp(-1.0, 1.0))
        .collect::<Vec<_>>()
}
