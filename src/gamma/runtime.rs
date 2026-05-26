mod narrative;
mod objective_runtime;

use sha2::{Digest, Sha256};

use crate::{
    beta::BetaRun, CapabilityId, ClaimRecord, ContractId, DeterminismPolicyId, ExecutionId,
    FailurePolicyId, GateDeclaration, GateResult, NumericPolicyId, OperatorDeclaration,
    OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken,
    ProbeExecutionRecord, ProvenancePolicyId, ReplayPolicyId, RunId, RuntimePolicyId,
    SideEffectPolicyId, SnapEdgeRef, SnapNodeRef, SnapPathRef, TraceId, TraceRecord, TraceStep,
    TraceStepId, UtcMinute, ValueRef,
};

use crate::SemanticError;

const GAMMA_PHASE: &str = "Γ";

#[derive(Clone, Debug, PartialEq)]
pub struct GammaObjectivePath {
    pub graph_declaration: OperatorDeclaration,
    pub graph_execution: OperatorExecutionRecord,
    pub graph_payload: PayloadRecord,
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub state_seed: Vec<f32>,
    pub runtime: GammaObjectiveRuntime,
    pub trace: TraceRecord,
    pub steps: Vec<TraceStep>,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaObjectiveRuntime {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub gate_declaration: GateDeclaration,
    pub gate_result: GateResult,
    pub claim: ClaimRecord,
    pub state_before: Vec<f32>,
    pub directed_phase_delta: Vec<f32>,
    pub wavelet_low_band: Vec<f32>,
    pub wavelet_high_band: Vec<f32>,
    pub wavelet_delta: Vec<f32>,
    pub recirculation_delta: Vec<f32>,
    pub reduction_state_after: Vec<f32>,
    pub state_after: Vec<f32>,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaNarrativePath {
    pub bridge_declaration: OperatorDeclaration,
    pub bridge_execution: OperatorExecutionRecord,
    pub bridge_payload: PayloadRecord,
    pub parcel_feedback: GammaNarrativeParcelFeedback,
    pub family_names: Vec<String>,
    pub family_mean_vector: Vec<f32>,
    pub trace: TraceRecord,
    pub steps: Vec<TraceStep>,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaNarrativeParcelFeedback {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub gate_declaration: GateDeclaration,
    pub gate_result: GateResult,
    pub vector: Vec<f32>,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaDualPathRuntime {
    pub objective_path: GammaObjectivePath,
    pub narrative_path: GammaNarrativePath,
}

pub fn run_gamma_dual_path_runtime(beta: &BetaRun) -> crate::SemanticResult<GammaDualPathRuntime> {
    let narrative_path = narrative::build_narrative_path(beta)?;

    Ok(GammaDualPathRuntime {
        objective_path: build_objective_path(beta, &narrative_path)?,
        narrative_path,
    })
}

fn build_objective_path(
    beta: &BetaRun,
    narrative_path: &GammaNarrativePath,
) -> Result<GammaObjectivePath, SemanticError> {
    let state_seed = build_objective_state_seed(beta);
    if state_seed.len() != beta.graph.node_count {
        return Err(SemanticError::new("gamma objective path must match beta parcel graph width"));
    }
    if state_seed.iter().any(|value| !value.is_finite()) {
        return Err(SemanticError::new("gamma objective path emitted non-finite parcel state"));
    }

    let graph_digest = sha256_hex(&[
        beta.graph.schema.as_bytes(),
        beta.graph.node_count.to_string().as_bytes(),
        beta.graph.hemisphere.join("|").as_bytes(),
    ]);
    let graph_declaration = OperatorDeclaration {
        id: OperatorId("operator-gamma-objective-beta-graph-source".into()),
        name: "gamma-objective-beta-graph-source".into(),
        inputs: Vec::new(),
        outputs: vec![ContractId("contract.graph.beta-360".into())],
        capabilities: vec![CapabilityId("capability.gamma-objective-beta-graph-source".into())],
        runtime: RuntimePolicyId("runtime.gamma.replayable".into()),
        determinism: DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
    };
    let graph_execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-gamma-objective-graph-{}", short_id(&graph_digest))),
        operator: graph_declaration.id.clone(),
        input_artifacts: Vec::new(),
        input_payloads: Vec::new(),
        output_payloads: vec![PayloadId(format!(
            "payload-gamma-objective-graph-{}",
            short_id(&graph_digest)
        ))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: graph_declaration.runtime.clone(),
        created: UtcMinute(202605250200),
    };
    let graph_payload = PayloadRecord {
        id: graph_execution.output_payloads[0].clone(),
        contract: ContractId("contract.graph.beta-360".into()),
        producer: graph_execution.id.clone(),
        source_artifacts: Vec::new(),
        source_payloads: Vec::new(),
        value: ValueRef("inline://beta/graph/360".into()),
        hash: Some(crate::HashDigest {
            algorithm: "sha256".into(),
            digest_hex: graph_digest,
        }),
        numeric: None,
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250200),
    };

    let digest = sha256_hex(&[
        canonical_vector(&state_seed).as_bytes(),
        beta.graph.schema.as_bytes(),
        beta.artifact.record.hash.digest_hex.as_bytes(),
    ]);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-gamma-objective-path".into()),
        name: "gamma-objective-path".into(),
        inputs: vec![
            ContractId("contract.artifact.alpha".into()),
            beta.embedding_probe.payload.contract.clone(),
            ContractId("contract.graph.beta-360".into()),
        ],
        outputs: vec![ContractId("contract.payload.gamma.objective-state-360".into())],
        capabilities: vec![CapabilityId("capability.gamma-objective-path".into())],
        runtime: RuntimePolicyId("runtime.gamma.replayable".into()),
        determinism: DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-gamma-objective-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: vec![beta.artifact.record.id.clone()],
        input_payloads: vec![beta.embedding_probe.payload.id.clone(), graph_payload.id.clone()],
        output_payloads: vec![PayloadId(format!("payload-gamma-objective-{}", short_id(&digest)))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605250201),
    };
    let payload = PayloadRecord {
        id: execution.output_payloads[0].clone(),
        contract: ContractId("contract.payload.gamma.objective-state-360".into()),
        producer: execution.id.clone(),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        source_payloads: vec![beta.embedding_probe.payload.id.clone(), graph_payload.id.clone()],
        value: ValueRef("inline://gamma/objective/state-360".into()),
        hash: Some(crate::HashDigest {
            algorithm: "sha256".into(),
            digest_hex: digest,
        }),
        numeric: Some(NumericPolicyId("numeric.signed-vector.360".into())),
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250201),
    };
    let mut trace = TraceRecord {
        id: TraceId(format!("trace-gamma-objective-{}", beta.artifact.record.id.0)),
        run: RunId(format!("run-gamma-{}", beta.artifact.record.id.0)),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        operator_executions: vec![
            beta.artifact.execution.id.clone(),
            beta.embedding_probe.execution.id.clone(),
            beta.embedding_probe.cache_execution.id.clone(),
            graph_execution.id.clone(),
            execution.id.clone(),
        ],
        payloads: vec![
            beta.embedding_probe.payload.id.clone(),
            graph_payload.id.clone(),
            payload.id.clone(),
        ],
        path: SnapPathRef(format!("snap://gamma/objective/{}", beta.artifact.record.id.0)),
        gate_results: Vec::new(),
        claims: Vec::new(),
        blocked_claims: Vec::new(),
        replay: ReplayPolicyId("replay.canonical".into()),
        created: UtcMinute(202605250201),
    };
    let mut steps = vec![
        trace_step(
            "trace-step-gamma-objective-intake",
            &trace.id,
            &beta.artifact.execution,
            vec![SnapNodeRef("g601".into())],
            vec![SnapEdgeRef("flow:g600->g601".into())],
        ),
        probe_trace_step(
            "trace-step-gamma-objective-embedding",
            &trace.id,
            &beta.embedding_probe.execution,
            vec![SnapNodeRef("g602".into())],
            vec![SnapEdgeRef("flow:g601->g602".into())],
        ),
        trace_step(
            "trace-step-gamma-objective-cache",
            &trace.id,
            &beta.embedding_probe.cache_execution,
            vec![SnapNodeRef("g603".into())],
            vec![SnapEdgeRef("flow:g602->g603".into())],
        ),
        trace_step(
            "trace-step-gamma-objective-graph",
            &trace.id,
            &graph_execution,
            vec![SnapNodeRef("g604".into())],
            vec![SnapEdgeRef("flow:g603->g604".into())],
        ),
        trace_step(
            "trace-step-gamma-objective-path",
            &trace.id,
            &execution,
            vec![SnapNodeRef("g605".into())],
            vec![SnapEdgeRef("flow:g604->g605".into())],
        ),
    ];
    let (runtime, runtime_step) = objective_runtime::build_objective_runtime(
        beta,
        &state_seed,
        &graph_payload,
        &payload,
        &narrative_path.parcel_feedback,
        &trace.id,
    )?;
    trace.operator_executions.push(narrative_path.bridge_execution.id.clone());
    trace.operator_executions.push(narrative_path.parcel_feedback.execution.id.clone());
    trace.operator_executions.push(runtime.execution.id.clone());
    trace.payloads.push(narrative_path.bridge_payload.id.clone());
    trace.payloads.push(narrative_path.parcel_feedback.payload.id.clone());
    trace.payloads.push(beta.gain.payload.id.clone());
    trace.payloads.push(beta.walk.payload.id.clone());
    trace.payloads.push(runtime.payload.id.clone());
    trace.gate_results.push(runtime.gate_result.gate_result_id.clone());
    trace.claims.push(runtime.claim.id.clone());
    steps.push(trace_step(
        "trace-step-gamma-objective-narrative-feedback",
        &trace.id,
        &narrative_path.parcel_feedback.execution,
        vec![SnapNodeRef("g616".into()), SnapNodeRef("g617".into())],
        vec![SnapEdgeRef("flow:g616->g617".into())],
    ));
    steps.push(runtime_step);

    Ok(GammaObjectivePath {
        graph_declaration,
        graph_execution,
        graph_payload,
        declaration,
        execution,
        payload,
        state_seed,
        runtime,
        trace,
        steps,
        detail: format!(
            "objective path seeds the {}-node parcel graph substrate from artifact-backed embedding features ({})",
            beta.graph.node_count, beta.graph.schema,
        ),
    })
}

fn build_objective_state_seed(beta: &BetaRun) -> Vec<f32> {
    (0..beta.graph.node_count)
        .map(|index| {
            let primary = beta.embedding_probe.values[index % beta.embedding_probe.values.len()];
            let secondary =
                beta.embedding_probe.values[(index * 5 + 3) % beta.embedding_probe.values.len()];
            let hemi = if beta.graph.hemisphere[index] == "L" {
                -0.05
            } else {
                0.05
            };
            ((primary * 0.8) + (secondary * 0.15) + hemi).clamp(-1.0, 1.0)
        })
        .collect::<Vec<_>>()
}

pub(super) fn probe_trace_step(
    id: &str,
    trace_id: &TraceId,
    execution: &ProbeExecutionRecord,
    snap_nodes: Vec<SnapNodeRef>,
    snap_edges: Vec<SnapEdgeRef>,
) -> TraceStep {
    TraceStep {
        id: TraceStepId(id.into()),
        trace: trace_id.clone(),
        operator: OperatorId(execution.probe.0.clone()),
        input_payloads: execution.input_payloads.clone(),
        output_payloads: execution.output_payloads.clone(),
        snap_nodes,
        snap_edges,
        gate_results: Vec::new(),
        started: Some(execution.created),
        finished: Some(execution.created),
    }
}

pub(super) fn trace_step(
    id: &str,
    trace_id: &TraceId,
    execution: &OperatorExecutionRecord,
    snap_nodes: Vec<SnapNodeRef>,
    snap_edges: Vec<SnapEdgeRef>,
) -> TraceStep {
    TraceStep {
        id: TraceStepId(id.into()),
        trace: trace_id.clone(),
        operator: execution.operator.clone(),
        input_payloads: execution.input_payloads.clone(),
        output_payloads: execution.output_payloads.clone(),
        snap_nodes,
        snap_edges,
        gate_results: Vec::new(),
        started: Some(execution.created),
        finished: Some(execution.created),
    }
}

pub(super) fn canonical_vector(values: &[f32]) -> String {
    values.iter().map(|value| format!("{value:+.6}")).collect::<Vec<_>>().join(",")
}

pub(super) fn sha256_hex(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part);
    }
    format!("{:x}", hasher.finalize())
}

pub(super) fn short_id(hex: &str) -> String {
    hex.chars().take(12).collect()
}
