use crate::{
    beta::BetaRun, CapabilityId, ContractId, DeterminismPolicyId, ExecutionId, FailurePolicyId,
    NumericPolicyId, OperatorDeclaration, OperatorExecutionRecord, OperatorId, PayloadId,
    PayloadRecord, PhaseToken, ProvenancePolicyId, ReplayPolicyId, RunId, RuntimePolicyId,
    SideEffectPolicyId, SnapEdgeRef, SnapNodeRef, SnapPathRef, TraceId, TraceRecord, UtcMinute,
    ValueRef,
};

use super::{
    canonical_vector, probe_trace_step, sha256_hex, short_id, trace_step, GammaError,
    GammaNarrativePath, GAMMA_PHASE,
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
        ],
        payloads: vec![
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.vibes.payload_11d.id.clone(),
            beta.gain.payload.id.clone(),
            bridge_payload.id.clone(),
        ],
        path: SnapPathRef(format!("snap://gamma/narrative/{}", beta.artifact.record.id.0)),
        gate_results: Vec::new(),
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
    ];

    Ok(GammaNarrativePath {
        bridge_declaration,
        bridge_execution,
        bridge_payload,
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
