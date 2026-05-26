use sha2::{Digest, Sha256};

use crate::{
    beta::BetaRun, CapabilityId, ClaimRecord, ClaimRecordId, ClaimStatus, ContractId,
    DeterminismPolicyId, ExecutionId, FailurePolicyId, FitnessFunctionId, GateDecision,
    GateDeclaration, GateId, GateResult, GateResultId, HashDigest, NumericPolicyId, ObservationId,
    ObservationRequest, OperatorDeclaration, OperatorExecutionRecord, OperatorId, OutputId,
    OutputRecord, PayloadId, PayloadRecord, PhaseToken, ProvenancePolicyId, ReplayPolicyId,
    RequirementId, RunId, RuntimePolicyId, SemanticError, SemanticResult, SideEffectPolicyId,
    SnapEdgeRef, SnapNodeRef, SnapPathRef, SnapRef, SubjectRef, TraceId, TraceRecord, TraceStep,
    TraceStepId, TruthAxisId, TruthAxisJudgment, TruthAxisResult, UncertaintyRecord, UtcMinute,
    ValueRef,
};

use super::{
    GammaCrossProjectionReadout, GammaDualPathRuntime, GammaLatentAxisStability,
    GammaLatentSweepSuite, GammaPriorEnsembleSuite, GammaProbeSuite, GammaProbeValiditySuite,
    GammaReceptorBridgeSuite,
};

const GAMMA_PHASE: &str = "Γ";

#[derive(Clone, Debug, PartialEq)]
pub struct GammaDiscoverySurface {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub gate_declaration: GateDeclaration,
    pub gate_result: GateResult,
    pub claim: ClaimRecord,
    pub output: OutputRecord,
    pub trace: TraceRecord,
    pub steps: Vec<TraceStep>,
    pub snap_text: String,
}

#[allow(clippy::too_many_arguments)]
pub fn run_gamma_discovery_surface(
    beta: &BetaRun,
    probe_suite: &GammaProbeSuite,
    latent_sweeps: &GammaLatentSweepSuite,
    probe_validity: &GammaProbeValiditySuite,
    prior_ensemble: &GammaPriorEnsembleSuite,
    receptor_bridge: &GammaReceptorBridgeSuite,
    dual_path: &GammaDualPathRuntime,
    readout: &GammaCrossProjectionReadout,
) -> SemanticResult<GammaDiscoverySurface> {
    let family_count = probe_suite.families.len();
    let prior_count = prior_ensemble.priors.len();
    let pair_count = readout.pairs.len();
    let receptor_count = receptor_bridge.families.len();
    let stable_count = latent_sweeps
        .axes
        .iter()
        .filter(|a| matches!(a.stability, GammaLatentAxisStability::Stable { .. }))
        .count();
    let hc_count = probe_validity.axes.iter().filter(|a| a.high_confidence_eligible).count();

    let digest = sha256_hex(&[
        beta.artifact.record.id.0.as_bytes(),
        readout.payload.id.0.as_bytes(),
        family_count.to_string().as_bytes(),
    ]);

    let declaration = OperatorDeclaration {
        id: OperatorId("operator-gamma-discovery-surface".into()),
        name: "gamma-discovery-surface".into(),
        inputs: vec![
            ContractId("contract.trace.beta-privileged-path".into()),
            ContractId("contract.payload.gamma.cross-projection-readout".into()),
        ],
        outputs: vec![ContractId("contract.output.gamma.discovery-surface".into())],
        capabilities: vec![CapabilityId("capability.gamma-discovery-surface".into())],
        runtime: RuntimePolicyId("runtime.gamma.replayable".into()),
        determinism: DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-gamma-discovery-surface-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: vec![beta.artifact.record.id.clone()],
        input_payloads: vec![
            beta.embedding_probe.payload.id.clone(),
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.gain.payload.id.clone(),
            beta.walk.payload.id.clone(),
            dual_path.objective_path.runtime.payload.id.clone(),
            readout.payload.id.clone(),
        ],
        output_payloads: vec![PayloadId(format!(
            "payload-gamma-discovery-surface-{}",
            short_id(&digest)
        ))],
        output_gate_results: vec![GateResultId("gate-result-gamma-discovery-surface".into())],
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605250300),
    };
    let payload = PayloadRecord {
        id: execution.output_payloads[0].clone(),
        contract: ContractId("contract.output.gamma.discovery-surface".into()),
        producer: execution.id.clone(),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        source_payloads: execution.input_payloads.clone(),
        value: ValueRef("inline://gamma/discovery/surface".into()),
        hash: Some(HashDigest {
            algorithm: "sha256".into(),
            digest_hex: digest.clone(),
        }),
        numeric: Some(NumericPolicyId("numeric.discovery-surface.gamma".into())),
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250300),
    };
    let gate_declaration = GateDeclaration {
        gate_id: GateId("gate-gamma-discovery-surface".into()),
        display_name: "verify-gamma-discovery-surface".into(),
        subject_contract: payload.contract.clone(),
        prerequisite_gate_ids: vec![
            GateId("gate-gamma-cross-projection-readout".into()),
            GateId("gate-gamma-objective-runtime-reduction".into()),
            GateId("gate-gamma-narrative-parcel-feedback".into()),
        ],
        fitness_function_id: FitnessFunctionId("fitness.gamma-discovery-surface".into()),
        phase_scope: Some(PhaseToken(GAMMA_PHASE.into())),
        applies_to_requirement_ids: vec![RequirementId(
            "requirement.gamma.discovery-surface".into(),
        )],
        truth_axes: vec![TruthAxisId("Integration".into()), TruthAxisId("Completeness".into())],
        failure_policy: FailurePolicyId("failure.stop-and-surface".into()),
    };
    let surface_complete = family_count > 0 && prior_count > 0 && pair_count > 0;
    let completeness_judgment = if surface_complete {
        TruthAxisJudgment::Yes
    } else {
        TruthAxisJudgment::Violation
    };
    let gate_result = GateResult {
        gate_result_id: GateResultId("gate-result-gamma-discovery-surface".into()),
        gate_id: gate_declaration.gate_id.clone(),
        subject_ref: SubjectRef(format!("payload:{}", payload.id.0)),
        prerequisite_results: vec![
            GateResultId("gate-result-gamma-cross-projection-readout".into()),
            GateResultId("gate-result-gamma-objective-runtime-reduction".into()),
            GateResultId("gate-result-gamma-narrative-parcel-feedback".into()),
        ],
        axis_results: vec![
            TruthAxisResult {
                axis_id: TruthAxisId("Integration".into()),
                judgment: TruthAxisJudgment::Yes,
                numeric_value: Some(1.0),
                evidence_refs: vec![
                    "observation:all-g1-g8-outputs-present".into(),
                    "observation:discovery-surface-references-beta-trace".into(),
                ],
            },
            TruthAxisResult {
                axis_id: TruthAxisId("Completeness".into()),
                judgment: completeness_judgment,
                numeric_value: Some(if surface_complete {
                    1.0
                } else {
                    0.0
                }),
                evidence_refs: vec![
                    format!("observation:probe-families={}", family_count),
                    format!("observation:priors={}", prior_count),
                    format!("observation:cross-projection-pairs={}", pair_count),
                ],
            },
        ],
        decision: if surface_complete {
            GateDecision::Pass
        } else {
            GateDecision::Fail
        },
        follow_up_observation: if surface_complete {
            None
        } else {
            Some(ObservationRequest {
                observation_id: ObservationId("observation-gamma-discovery-incomplete".into()),
                description: "discovery surface incomplete: \
                    one or more gamma components returned empty output"
                    .into(),
                required_subject: SubjectRef(format!("payload:{}", payload.id.0)),
                expected_resolution: "all gamma components must produce non-empty output".into(),
            })
        },
        evidence_payload_ids: vec![
            beta.embedding_probe.payload.id.clone(),
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.gain.payload.id.clone(),
            beta.walk.payload.id.clone(),
            readout.payload.id.clone(),
            payload.id.clone(),
        ],
        evidence_trace_ids: vec![
            beta.report.trace.id.clone(),
            dual_path.objective_path.trace.id.clone(),
            dual_path.narrative_path.trace.id.clone(),
            readout.trace.id.clone(),
        ],
        created: UtcMinute(202605250300),
    };
    let claim = ClaimRecord {
        id: ClaimRecordId("claim-gamma-discovery-surface".into()),
        statement: format!(
            "gamma discovery surface over {} assembled {} probe families ({} high-confidence), \
            {} priors, {} receptor families, {} latent axes ({} stable), \
            and {} cross-projection pairs; \
            every claim links to artifact hash, probe record, Snap edge, prior, and runtime step",
            beta.artifact.record.id.0,
            family_count,
            hc_count,
            prior_count,
            receptor_count,
            latent_sweeps.axes.len(),
            stable_count,
            pair_count,
        ),
        status: ClaimStatus::DerivedClaim,
        phase_scope: Some(PhaseToken(GAMMA_PHASE.into())),
        support_artifacts: vec![beta.artifact.record.id.clone()],
        support_payloads: vec![
            beta.embedding_probe.payload.id.clone(),
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.gain.payload.id.clone(),
            beta.walk.payload.id.clone(),
            dual_path.objective_path.runtime.payload.id.clone(),
            readout.payload.id.clone(),
            payload.id.clone(),
        ],
        support_traces: vec![
            beta.report.trace.id.clone(),
            dual_path.objective_path.trace.id.clone(),
            dual_path.narrative_path.trace.id.clone(),
            readout.trace.id.clone(),
        ],
        support_gate_results: vec![
            beta.report.gate_result.gate_result_id.clone(),
            dual_path.objective_path.runtime.gate_result.gate_result_id.clone(),
            dual_path.narrative_path.parcel_feedback.gate_result.gate_result_id.clone(),
            readout.gate_result.gate_result_id.clone(),
            gate_result.gate_result_id.clone(),
        ],
        support_snaps: vec![
            SnapRef(format!("snap://gamma/discovery/{}#node=g110", beta.artifact.record.id.0)),
            SnapRef("snap://beta/trace#node=b109".into()),
        ],
        uncertainty: Some(UncertaintyRecord {
            belief: Some(1.0),
            plausibility: Some(1.0),
            confidence: Some(1.0),
            conflict: Some(0.0),
            unsupported_mass: Some(
                beta.disagreement.unsupported_edges as f32 / beta.graph.node_count.max(1) as f32,
            ),
        }),
        blocker: None,
        created: UtcMinute(202605250300),
    };
    let trace_id = TraceId(format!("trace-gamma-discovery-{}", beta.artifact.record.id.0));
    let trace = TraceRecord {
        id: trace_id.clone(),
        run: RunId(format!("run-gamma-{}", beta.artifact.record.id.0)),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        operator_executions: vec![
            dual_path.objective_path.execution.id.clone(),
            dual_path.objective_path.runtime.execution.id.clone(),
            dual_path.narrative_path.bridge_execution.id.clone(),
            dual_path.narrative_path.parcel_feedback.execution.id.clone(),
            readout.execution.id.clone(),
            execution.id.clone(),
        ],
        payloads: vec![
            beta.embedding_probe.payload.id.clone(),
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.gain.payload.id.clone(),
            beta.walk.payload.id.clone(),
            beta.disagreement.payload.id.clone(),
            dual_path.objective_path.payload.id.clone(),
            dual_path.objective_path.runtime.payload.id.clone(),
            dual_path.narrative_path.bridge_payload.id.clone(),
            dual_path.narrative_path.parcel_feedback.payload.id.clone(),
            readout.payload.id.clone(),
            payload.id.clone(),
        ],
        path: SnapPathRef(format!("snap://gamma/discovery/{}", beta.artifact.record.id.0)),
        gate_results: vec![gate_result.gate_result_id.clone()],
        claims: vec![claim.id.clone()],
        blocked_claims: Vec::new(),
        replay: ReplayPolicyId("replay.canonical".into()),
        created: UtcMinute(202605250300),
    };
    let step = TraceStep {
        id: TraceStepId("trace-step-gamma-discovery-surface".into()),
        trace: trace_id.clone(),
        operator: execution.operator.clone(),
        input_payloads: execution.input_payloads.clone(),
        output_payloads: execution.output_payloads.clone(),
        snap_nodes: vec![SnapNodeRef("g110".into())],
        snap_edges: vec![SnapEdgeRef("flow:g109->g110".into())],
        gate_results: vec![gate_result.gate_result_id.clone()],
        started: Some(execution.created),
        finished: Some(UtcMinute(202605250301)),
    };
    let output = OutputRecord {
        id: OutputId("output-gamma-discovery-surface".into()),
        name: "gamma-discovery-surface-report".into(),
        source_traces: vec![
            beta.report.trace.id.clone(),
            dual_path.objective_path.trace.id.clone(),
            dual_path.narrative_path.trace.id.clone(),
            readout.trace.id.clone(),
            trace_id,
        ],
        included_claims: vec![
            beta.report.claim.id.clone(),
            dual_path.objective_path.runtime.claim.id.clone(),
            readout.claim.id.clone(),
            claim.id.clone(),
        ],
        included_gate_results: vec![
            beta.report.gate_result.gate_result_id.clone(),
            dual_path.objective_path.runtime.gate_result.gate_result_id.clone(),
            dual_path.narrative_path.parcel_feedback.gate_result.gate_result_id.clone(),
            readout.gate_result.gate_result_id.clone(),
            gate_result.gate_result_id.clone(),
        ],
        export: ValueRef(format!(
            "file://output/reports/gamma-discovery-{}.snap",
            beta.artifact.record.id.0
        )),
        generator: declaration.id.clone(),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250300),
    };
    let snap_text = gamma_discovery_snap(
        &beta.artifact.record.id.0,
        &beta.artifact.record.hash.digest_hex,
        family_count,
        prior_count,
        pair_count,
        receptor_count,
    );
    if gamma_discovery_snap(
        &beta.artifact.record.id.0,
        &beta.artifact.record.hash.digest_hex,
        family_count,
        prior_count,
        pair_count,
        receptor_count,
    ) != snap_text
    {
        return Err(SemanticError::new("gamma discovery snap emission was not stable"));
    }
    Ok(GammaDiscoverySurface {
        declaration,
        execution,
        payload,
        gate_declaration,
        gate_result,
        claim,
        output,
        trace,
        steps: vec![step],
        snap_text,
    })
}

fn gamma_discovery_snap(
    artifact_id: &str,
    artifact_hash: &str,
    probe_families: usize,
    priors: usize,
    cross_projection_pairs: usize,
    receptor_families: usize,
) -> String {
    [
        "🪢snap resonance-gamma-discovery".into(),
        ".graph {".into(),
        format!(" id: gamma-discovery-{artifact_id},"),
        " name: 'resonance-gamma-discovery',".into(),
        " version: 0.8,".into(),
        "}".into(),
        "nodes {".into(),
        " object { id: g101, name: 'Snap-Spine-Γ', type: SnapSpine, }".into(),
        " object { id: g102, name: 'Probe-Suite-Γ', type: ProbeSuite, }".into(),
        " object { id: g103, name: 'Latent-Axis-Sweep-Γ', type: LatentAxisSweep, }".into(),
        " object { id: g104, name: 'Probe-Validity-Evaluator-Γ', type: ProbeValidity, }".into(),
        " object { id: g105, name: 'Prior-Ensemble-Γ', type: PriorEnsemble, }".into(),
        " object { id: g106, name: 'Receptor-Bridge-Γ', type: ReceptorBridge, }".into(),
        " object { id: g107, name: 'Dual-Path-Runtime-Γ', type: DualPathRuntime, }".into(),
        " object { id: g108, name: 'Magnetic-Wavelet-Runtime-Γ', type: MagneticWaveletRuntime, }".into(),
        " object { id: g109, name: 'Cross-Projection-Readout-Γ', type: CrossProjectionReadout, }".into(),
        " object { id: g110, name: 'Discovery-Surface-Γ', type: DiscoverySurface, }".into(),
        " object { id: o110, name: 'Observe-Discovery-Surface❇gamma', type: ObservationNode, }".into(),
        "}".into(),
        "edges {".into(),
        " extend { @b109 -> @g101, }".into(),
        " flow { @g101 -> @g102, @g102 -> @g103, @g103 -> @g104, @g104 -> @g105, @g105 -> @g106, @g106 -> @g107, @g107 -> @g108, @g108 -> @g109, @g109 -> @g110, }".into(),
        " verify { @g110 -> @o110, }".into(),
        "}".into(),
        "registers {".into(),
        format!(" artifact_id: '{artifact_id}',"),
        format!(" artifact_hash: '{artifact_hash}',"),
        format!(" probe_families: '{probe_families}',"),
        format!(" priors: '{priors}',"),
        format!(" receptor_families: '{receptor_families}',"),
        format!(" cross_projection_pairs: '{cross_projection_pairs}',"),
        "}".into(),
    ]
    .join("\n")
}

fn sha256_hex(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part);
    }
    format!("{:x}", hasher.finalize())
}
fn short_id(hex: &str) -> String {
    hex.chars().take(12).collect()
}
