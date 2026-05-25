use crate::{
    beta::BetaRun, CapabilityId, ClaimRecord, ClaimRecordId, ClaimStatus, ContractId,
    FailurePolicyId, FitnessFunctionId, GateDecision, GateDeclaration, GateId, GateResult,
    GateResultId, NumericPolicyId, OperatorDeclaration, OperatorExecutionRecord, OperatorId,
    PayloadId, PayloadRecord, PhaseToken, ProvenancePolicyId, RequirementId, RuntimePolicyId,
    SideEffectPolicyId, SnapEdgeRef, SnapNodeRef, SnapRef, SubjectRef, TraceId, TraceStep,
    TraceStepId, TruthAxisId, TruthAxisJudgment, TruthAxisResult, UtcMinute, ValueRef,
};

use super::{
    canonical_vector, sha256_hex, short_id, GammaError, GammaNarrativeParcelFeedback,
    GammaObjectiveRuntime, GAMMA_PHASE,
};

const DIRECTED_PHASE_STRENGTH: f32 = 0.075;
const WAVELET_HIGH_BAND_STRENGTH: f32 = 0.12;
const RECIRCULATION_STRENGTH: f32 = 0.03;

struct RuntimeControls {
    directed_phase_strength: f32,
    wavelet_high_band_strength: f32,
    recirculation_strength: f32,
}

struct RuntimeProjection {
    state_before: Vec<f32>,
    directed_phase_delta: Vec<f32>,
    wavelet_low_band: Vec<f32>,
    wavelet_high_band: Vec<f32>,
    wavelet_delta: Vec<f32>,
    recirculation_delta: Vec<f32>,
    state_after: Vec<f32>,
}

pub(super) fn build_objective_runtime(
    beta: &BetaRun,
    state_seed: &[f32],
    graph_payload: &PayloadRecord,
    objective_payload: &PayloadRecord,
    narrative_feedback: &GammaNarrativeParcelFeedback,
    trace_id: &TraceId,
) -> Result<(GammaObjectiveRuntime, TraceStep), GammaError> {
    let runtime_projection = project_runtime(
        beta,
        state_seed,
        &narrative_feedback.vector,
        RuntimeControls {
            directed_phase_strength: DIRECTED_PHASE_STRENGTH,
            wavelet_high_band_strength: WAVELET_HIGH_BAND_STRENGTH,
            recirculation_strength: RECIRCULATION_STRENGTH,
        },
    )?;
    let reduction_projection = project_runtime(
        beta,
        state_seed,
        &narrative_feedback.vector,
        RuntimeControls {
            directed_phase_strength: 0.0,
            wavelet_high_band_strength: 0.0,
            recirculation_strength: 0.0,
        },
    )?;
    if reduction_projection.state_after != beta.walk.state_after {
        return Err(GammaError::new(
            "gamma objective runtime must reduce to the verified beta laplacian runtime",
        ));
    }

    let digest = sha256_hex(&[
        canonical_vector(&runtime_projection.state_before).as_bytes(),
        canonical_vector(&runtime_projection.directed_phase_delta).as_bytes(),
        canonical_vector(&runtime_projection.wavelet_delta).as_bytes(),
        canonical_vector(&runtime_projection.recirculation_delta).as_bytes(),
        canonical_vector(&runtime_projection.state_after).as_bytes(),
    ]);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-gamma-objective-runtime".into()),
        name: "gamma-objective-runtime".into(),
        inputs: vec![
            graph_payload.contract.clone(),
            objective_payload.contract.clone(),
            beta.walk.payload.contract.clone(),
            beta.gain.payload.contract.clone(),
            narrative_feedback.payload.contract.clone(),
        ],
        outputs: vec![ContractId("contract.payload.gamma.objective-runtime-state-360".into())],
        capabilities: vec![CapabilityId("capability.gamma-objective-runtime".into())],
        runtime: RuntimePolicyId("runtime.gamma.replayable".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: crate::ExecutionId(format!("execution-gamma-objective-runtime-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: vec![beta.artifact.record.id.clone()],
        input_payloads: vec![
            graph_payload.id.clone(),
            objective_payload.id.clone(),
            beta.walk.payload.id.clone(),
            beta.gain.payload.id.clone(),
            narrative_feedback.payload.id.clone(),
        ],
        output_payloads: vec![PayloadId(format!(
            "payload-gamma-objective-runtime-{}",
            short_id(&digest)
        ))],
        output_gate_results: vec![GateResultId(
            "gate-result-gamma-objective-runtime-reduction".into(),
        )],
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605250202),
    };
    let payload = PayloadRecord {
        id: execution.output_payloads[0].clone(),
        contract: ContractId("contract.payload.gamma.objective-runtime-state-360".into()),
        producer: execution.id.clone(),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        source_payloads: execution.input_payloads.clone(),
        value: ValueRef("inline://gamma/objective/runtime-state-360".into()),
        hash: Some(crate::HashDigest {
            algorithm: "sha256".into(),
            digest_hex: digest,
        }),
        numeric: Some(NumericPolicyId("numeric.signed-vector.360".into())),
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250202),
    };
    let gate_declaration = GateDeclaration {
        gate_id: GateId("gate-gamma-objective-runtime-reduction".into()),
        display_name: "verify-gamma-objective-runtime-reduction".into(),
        subject_contract: payload.contract.clone(),
        prerequisite_gate_ids: vec![narrative_feedback.gate_declaration.gate_id.clone()],
        fitness_function_id: FitnessFunctionId("fitness.gamma-objective-runtime-reduction".into()),
        phase_scope: Some(PhaseToken(GAMMA_PHASE.into())),
        applies_to_requirement_ids: vec![RequirementId(
            "requirement.gamma.runtime-reduction".into(),
        )],
        truth_axes: vec![TruthAxisId("Integration".into()), TruthAxisId("Performance".into())],
        failure_policy: FailurePolicyId("failure.stop-and-surface".into()),
    };
    let gate_result = GateResult {
        gate_result_id: GateResultId("gate-result-gamma-objective-runtime-reduction".into()),
        gate_id: gate_declaration.gate_id.clone(),
        subject_ref: SubjectRef(format!("payload:{}", payload.id.0)),
        prerequisite_results: vec![narrative_feedback.gate_result.gate_result_id.clone()],
        axis_results: vec![
            TruthAxisResult {
                axis_id: TruthAxisId("Integration".into()),
                judgment: TruthAxisJudgment::Yes,
                numeric_value: Some(1.0),
                evidence_refs: vec![
                    "observation:runtime-inputs-are-declared".into(),
                    "observation:narrative-parcel-feedback-is-traced".into(),
                ],
            },
            TruthAxisResult {
                axis_id: TruthAxisId("Performance".into()),
                judgment: TruthAxisJudgment::Yes,
                numeric_value: Some(1.0),
                evidence_refs: vec![
                    "observation:gamma-objective-runtime-reduces-to-beta".into(),
                    "observation:wavelet-filter-reduction-replays-beta".into(),
                    "observation:gamma-only-deltas-zero-in-reduction-proof".into(),
                ],
            },
        ],
        decision: GateDecision::Pass,
        follow_up_observation: None,
        evidence_payload_ids: vec![
            graph_payload.id.clone(),
            objective_payload.id.clone(),
            beta.walk.payload.id.clone(),
            beta.gain.payload.id.clone(),
            narrative_feedback.payload.id.clone(),
            payload.id.clone(),
        ],
        evidence_trace_ids: vec![trace_id.clone()],
        created: UtcMinute(202605250202),
    };
    let claim = ClaimRecord {
        id: ClaimRecordId("claim-gamma-objective-runtime-reduction".into()),
        statement: format!(
            "Gamma objective runtime emits traced directed-phase, wavelet, and recirculation carriers over {} parcels, and the reduction proof for the same kernel collapses to the verified beta laplacian runtime when gamma-only terms are disabled",
            beta.graph.node_count,
        ),
        status: ClaimStatus::DerivedClaim,
        phase_scope: Some(PhaseToken(GAMMA_PHASE.into())),
        support_artifacts: vec![beta.artifact.record.id.clone()],
        support_payloads: vec![
            graph_payload.id.clone(),
            objective_payload.id.clone(),
            narrative_feedback.payload.id.clone(),
            beta.walk.payload.id.clone(),
            beta.gain.payload.id.clone(),
            payload.id.clone(),
        ],
        support_traces: vec![trace_id.clone()],
        support_gate_results: vec![gate_result.gate_result_id.clone()],
        support_snaps: vec![SnapRef(format!(
            "snap://gamma/objective/{}#node=g606",
            beta.artifact.record.id.0,
        ))],
        uncertainty: None,
        blocker: None,
        created: UtcMinute(202605250202),
    };
    let step = TraceStep {
        id: TraceStepId("trace-step-gamma-objective-runtime".into()),
        trace: trace_id.clone(),
        operator: execution.operator.clone(),
        input_payloads: execution.input_payloads.clone(),
        output_payloads: execution.output_payloads.clone(),
        snap_nodes: vec![SnapNodeRef("g606".into()), SnapNodeRef("o107".into())],
        snap_edges: vec![
            SnapEdgeRef("flow:g605->g606".into()),
            SnapEdgeRef("flow:g617->g606".into()),
            SnapEdgeRef("verify:g606->o107".into()),
        ],
        gate_results: vec![gate_result.gate_result_id.clone()],
        started: Some(execution.created),
        finished: Some(execution.created),
    };

    Ok((
        GammaObjectiveRuntime {
            declaration,
            execution,
            payload,
            gate_declaration,
            gate_result,
            claim,
            state_before: runtime_projection.state_before,
            directed_phase_delta: runtime_projection.directed_phase_delta,
            wavelet_low_band: runtime_projection.wavelet_low_band,
            wavelet_high_band: runtime_projection.wavelet_high_band,
            wavelet_delta: runtime_projection.wavelet_delta,
            recirculation_delta: runtime_projection.recirculation_delta,
            reduction_state_after: reduction_projection.state_after,
            state_after: runtime_projection.state_after,
            detail: "objective runtime now carries directed phase edges, two-band wavelet filtering, and narrative recirculation while a reduction proof gate verifies the same kernel collapses to beta when gamma-only terms are disabled".into(),
        },
        step,
    ))
}

fn project_runtime(
    beta: &BetaRun,
    state_seed: &[f32],
    parcel_feedback_vector: &[f32],
    controls: RuntimeControls,
) -> Result<RuntimeProjection, GammaError> {
    if state_seed.len() != beta.graph.node_count {
        return Err(GammaError::new(
            "gamma objective runtime must preserve beta parcel graph width",
        ));
    }
    if parcel_feedback_vector.len() != beta.graph.node_count {
        return Err(GammaError::new("gamma objective runtime requires narrative family evidence"));
    }

    let state_before = beta.walk.state_before.clone();
    let phase_carrier = state_seed
        .iter()
        .zip(state_before.iter())
        .map(|(seed, state)| (seed - state).clamp(-1.0, 1.0))
        .collect::<Vec<_>>();
    let directed_phase_base = beta
        .graph
        .apply_directed_phase_term(&state_before, &phase_carrier)
        .map_err(GammaError::from)?;
    let directed_phase_delta = directed_phase_base
        .iter()
        .map(|value| controls.directed_phase_strength * value)
        .collect::<Vec<_>>();

    let wavelet_low_band = beta.walk.laplacian_delta.clone();
    let wavelet_high_band =
        normalize_vector(&beta.graph.apply_laplacian(&wavelet_low_band).map_err(GammaError::from)?);
    let wavelet_delta = wavelet_low_band
        .iter()
        .zip(wavelet_high_band.iter())
        .zip(phase_carrier.iter())
        .map(|((low, high), carrier)| {
            low + (controls.wavelet_high_band_strength * carrier.abs() * high)
        })
        .collect::<Vec<_>>();

    let recirculation_delta = parcel_feedback_vector
        .iter()
        .zip(beta.gain.vector.iter())
        .map(|(carrier, gain_value)| controls.recirculation_strength * carrier * gain_value)
        .collect::<Vec<_>>();

    let state_after = state_before
        .iter()
        .zip(wavelet_delta.iter())
        .zip(beta.gain.vector.iter())
        .zip(directed_phase_delta.iter())
        .zip(recirculation_delta.iter())
        .map(|((((state, delta), gain_value), directed), recirculation)| {
            state - (beta.walk.eta * delta) + gain_value + directed + recirculation
        })
        .collect::<Vec<_>>();
    if state_after.iter().any(|value| !value.is_finite()) {
        return Err(GammaError::new("gamma objective runtime emitted non-finite parcel state"));
    }

    Ok(RuntimeProjection {
        state_before,
        directed_phase_delta,
        wavelet_low_band,
        wavelet_high_band,
        wavelet_delta,
        recirculation_delta,
        state_after,
    })
}

fn normalize_vector(values: &[f32]) -> Vec<f32> {
    let max_abs = values.iter().fold(0.0_f32, |current, value| current.max(value.abs()));
    if max_abs <= f32::EPSILON {
        return vec![0.0; values.len()];
    }

    values.iter().map(|value| value / max_abs).collect::<Vec<_>>()
}
