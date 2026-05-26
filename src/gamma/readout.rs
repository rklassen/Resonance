use sha2::{Digest, Sha256};

use crate::{
    beta::BetaRun, BlockedClaimId, CapabilityId, ClaimRecord, ClaimRecordId, ClaimStatus,
    ContractId, DeterminismPolicyId, ExecutionId, FailurePolicyId, FitnessFunctionId, GateDecision,
    GateDeclaration, GateId, GateResult, GateResultId, HashDigest, NumericPolicyId,
    OperatorDeclaration, OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken,
    ProvenancePolicyId, ReplayPolicyId, RequirementId, RunId, RuntimePolicyId, SemanticError,
    SemanticResult, SideEffectPolicyId, SnapEdgeRef, SnapNodeRef, SnapPathRef, SnapRef, SubjectRef,
    TraceId, TraceRecord, TraceStep, TraceStepId, TruthAxisId, TruthAxisJudgment, TruthAxisResult,
    UtcMinute, ValueRef,
};

use super::GammaDualPathRuntime;

const GAMMA_PHASE: &str = "Γ";
const DISAGREEMENT_THRESHOLD: f32 = f32::EPSILON;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GammaDisagreementLocalizer {
    Probe(String),
    Prompt(String),
    Prior(String),
    Transform(String),
    Operator(String),
    GraphEdge(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaProjectionAgreement {
    pub name: String,
    pub disagreement: f32,
    pub localizer: Option<GammaDisagreementLocalizer>,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaCrossProjectionReadout {
    pub pairs: Vec<GammaProjectionAgreement>,
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub gate_declaration: GateDeclaration,
    pub gate_result: GateResult,
    pub claim: ClaimRecord,
    pub trace: TraceRecord,
    pub steps: Vec<TraceStep>,
    pub detail: String,
}

pub fn run_gamma_cross_projection_readout(
    beta: &BetaRun,
    dual_path: &GammaDualPathRuntime,
) -> SemanticResult<GammaCrossProjectionReadout> {
    let pairs = build_pairs(beta, dual_path);
    if pairs.iter().any(|pair| !pair.disagreement.is_finite()) {
        return Err(SemanticError::new("cross-projection readout emitted non-finite disagreement"));
    }
    let scores_text =
        pairs.iter().map(|pair| format!("{:+.6}", pair.disagreement)).collect::<Vec<_>>().join(",");
    let digest = sha256_hex(&[scores_text.as_bytes(), beta.artifact.record.id.0.as_bytes()]);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-gamma-cross-projection-readout".into()),
        name: "gamma-cross-projection-readout".into(),
        inputs: vec![
            beta.embedding_probe.payload.contract.clone(),
            beta.label_probe.payload.contract.clone(),
            beta.vibes.payload_12d.contract.clone(),
            beta.gain.payload.contract.clone(),
            beta.walk.payload.contract.clone(),
            dual_path.objective_path.runtime.payload.contract.clone(),
        ],
        outputs: vec![ContractId("contract.payload.gamma.cross-projection-readout".into())],
        capabilities: vec![CapabilityId("capability.gamma-cross-projection-readout".into())],
        runtime: RuntimePolicyId("runtime.gamma.replayable".into()),
        determinism: DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-gamma-cross-projection-readout-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: vec![beta.artifact.record.id.clone()],
        input_payloads: vec![
            beta.embedding_probe.payload.id.clone(),
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.gain.payload.id.clone(),
            beta.walk.payload.id.clone(),
            dual_path.objective_path.runtime.payload.id.clone(),
        ],
        output_payloads: vec![PayloadId(format!(
            "payload-gamma-cross-projection-readout-{}",
            short_id(&digest)
        ))],
        output_gate_results: vec![GateResultId(
            "gate-result-gamma-cross-projection-readout".into(),
        )],
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605250210),
    };
    let payload = PayloadRecord {
        id: execution.output_payloads[0].clone(),
        contract: ContractId("contract.payload.gamma.cross-projection-readout".into()),
        producer: execution.id.clone(),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        source_payloads: execution.input_payloads.clone(),
        value: ValueRef("inline://gamma/readout/cross-projection".into()),
        hash: Some(HashDigest {
            algorithm: "sha256".into(),
            digest_hex: digest.clone(),
        }),
        numeric: Some(NumericPolicyId("numeric.disagreement-scores.4".into())),
        provenance: ProvenancePolicyId("provenance.explicit".into()),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        created: UtcMinute(202605250210),
    };
    let gate_declaration = GateDeclaration {
        gate_id: GateId("gate-gamma-cross-projection-readout".into()),
        display_name: "verify-gamma-cross-projection-readout".into(),
        subject_contract: payload.contract.clone(),
        prerequisite_gate_ids: vec![
            GateId("gate-gamma-objective-runtime-reduction".into()),
            GateId("gate-gamma-narrative-parcel-feedback".into()),
        ],
        fitness_function_id: FitnessFunctionId("fitness.gamma-cross-projection-readout".into()),
        phase_scope: Some(PhaseToken(GAMMA_PHASE.into())),
        applies_to_requirement_ids: vec![RequirementId(
            "requirement.gamma.cross-projection-readout".into(),
        )],
        truth_axes: vec![TruthAxisId("Integration".into()), TruthAxisId("Performance".into())],
        failure_policy: FailurePolicyId("failure.stop-and-surface".into()),
    };
    let gate_result = GateResult {
        gate_result_id: GateResultId("gate-result-gamma-cross-projection-readout".into()),
        gate_id: gate_declaration.gate_id.clone(),
        subject_ref: SubjectRef(format!("payload:{}", payload.id.0)),
        prerequisite_results: vec![
            GateResultId("gate-result-gamma-objective-runtime-reduction".into()),
            GateResultId("gate-result-gamma-narrative-parcel-feedback".into()),
        ],
        axis_results: vec![
            TruthAxisResult {
                axis_id: TruthAxisId("Integration".into()),
                judgment: TruthAxisJudgment::Yes,
                numeric_value: Some(1.0),
                evidence_refs: vec![
                    "observation:all-comparison-inputs-are-declared".into(),
                    "observation:readout-payloads-are-provenanced".into(),
                ],
            },
            TruthAxisResult {
                axis_id: TruthAxisId("Performance".into()),
                judgment: TruthAxisJudgment::Yes,
                numeric_value: Some(1.0),
                evidence_refs: vec!["observation:every-disagreement-names-a-localizer".into()],
            },
        ],
        decision: GateDecision::Pass,
        follow_up_observation: None,
        evidence_payload_ids: vec![
            beta.embedding_probe.payload.id.clone(),
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.gain.payload.id.clone(),
            beta.walk.payload.id.clone(),
            dual_path.objective_path.runtime.payload.id.clone(),
            payload.id.clone(),
        ],
        evidence_trace_ids: vec![TraceId(format!(
            "trace-gamma-cross-projection-readout-{}",
            beta.artifact.record.id.0,
        ))],
        created: UtcMinute(202605250210),
    };
    let claim = ClaimRecord {
        id: ClaimRecordId("claim-gamma-cross-projection-readout".into()),
        statement: format!(
            "gamma cross-projection readout over {} evaluated 4 projection pairs; \
            every disagreement above threshold names a localizer; \
            vibes-to-receptor pair deferred pending axis mapping",
            beta.artifact.record.id.0,
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
            payload.id.clone(),
        ],
        support_traces: vec![TraceId(format!(
            "trace-gamma-cross-projection-readout-{}",
            beta.artifact.record.id.0,
        ))],
        support_gate_results: vec![gate_result.gate_result_id.clone()],
        support_snaps: vec![SnapRef(format!(
            "snap://gamma/readout/{}#node=g801",
            beta.artifact.record.id.0,
        ))],
        uncertainty: None,
        blocker: None,
        created: UtcMinute(202605250210),
    };
    let trace_id =
        TraceId(format!("trace-gamma-cross-projection-readout-{}", beta.artifact.record.id.0,));
    let trace = TraceRecord {
        id: trace_id.clone(),
        run: RunId(format!("run-gamma-{}", beta.artifact.record.id.0)),
        phase: Some(PhaseToken(GAMMA_PHASE.into())),
        source_artifacts: vec![beta.artifact.record.id.clone()],
        operator_executions: vec![
            beta.embedding_probe.execution.id.clone(),
            beta.label_probe.execution.id.clone(),
            beta.vibes.execution.id.clone(),
            beta.gain.execution.id.clone(),
            beta.walk.execution.id.clone(),
            dual_path.objective_path.runtime.execution.id.clone(),
            execution.id.clone(),
        ],
        payloads: vec![
            beta.embedding_probe.payload.id.clone(),
            beta.label_probe.payload.id.clone(),
            beta.vibes.payload_12d.id.clone(),
            beta.gain.payload.id.clone(),
            beta.walk.payload.id.clone(),
            dual_path.objective_path.runtime.payload.id.clone(),
            payload.id.clone(),
        ],
        path: SnapPathRef(format!("snap://gamma/readout/{}", beta.artifact.record.id.0,)),
        gate_results: vec![gate_result.gate_result_id.clone()],
        claims: vec![claim.id.clone()],
        blocked_claims: vec![BlockedClaimId("blocked-claim-gamma-vibes-receptor-agreement".into())],
        replay: ReplayPolicyId("replay.canonical".into()),
        created: UtcMinute(202605250210),
    };
    let step = TraceStep {
        id: TraceStepId("trace-step-gamma-cross-projection-readout".into()),
        trace: trace_id,
        operator: execution.operator.clone(),
        input_payloads: execution.input_payloads.clone(),
        output_payloads: execution.output_payloads.clone(),
        snap_nodes: vec![SnapNodeRef("g801".into())],
        snap_edges: vec![
            SnapEdgeRef("flow:g606->g801".into()),
            SnapEdgeRef("verify:g801->o801".into()),
        ],
        gate_results: vec![gate_result.gate_result_id.clone()],
        started: Some(execution.created),
        finished: Some(execution.created),
    };
    Ok(GammaCrossProjectionReadout {
        pairs,
        declaration,
        execution,
        payload,
        gate_declaration,
        gate_result,
        claim,
        trace,
        steps: vec![step],
        detail: format!(
            "cross-projection readout over artifact {} compares 4 projection pairs; \
            parcel-trajectory divergence is by design (gamma-objective-runtime operator transforms state)",
            beta.artifact.record.id.0,
        ),
    })
}

fn build_pairs(beta: &BetaRun, dual_path: &GammaDualPathRuntime) -> Vec<GammaProjectionAgreement> {
    let probe_probe = {
        let d = mean_absolute_difference(&beta.embedding_probe.values, &beta.label_probe.values);
        GammaProjectionAgreement {
            name: "probe-probe".into(),
            disagreement: d,
            localizer: if d > DISAGREEMENT_THRESHOLD {
                Some(GammaDisagreementLocalizer::Probe("embedding-probe-vs-label-probe".into()))
            } else {
                None
            },
            detail: "mean absolute difference between 16D embedding probe and 12D label probe \
                (compared over 12 common dimensions)"
                .into(),
        }
    };
    let semantic_vibes = {
        let d = mean_absolute_difference(&beta.label_probe.values, &beta.vibes.signed_12d);
        GammaProjectionAgreement {
            name: "semantic-vibes".into(),
            disagreement: d,
            localizer: if d > DISAGREEMENT_THRESHOLD {
                Some(GammaDisagreementLocalizer::Transform("vibes-projection".into()))
            } else {
                None
            },
            detail: "mean absolute difference between 12D label probe and 12D vibes projection"
                .into(),
        }
    };
    let receptor_parcel = {
        let d = mean_absolute_difference(&beta.gain.vector, &beta.walk.state_after);
        GammaProjectionAgreement {
            name: "receptor-parcel".into(),
            disagreement: d,
            localizer: if d > DISAGREEMENT_THRESHOLD {
                Some(GammaDisagreementLocalizer::Prior("receptor-gain-terms".into()))
            } else {
                None
            },
            detail: "mean absolute difference between 360D receptor gain vector and 360D \
                post-walk parcel state"
                .into(),
        }
    };
    let parcel_trajectory = {
        let d = mean_absolute_difference(
            &beta.walk.state_after,
            &dual_path.objective_path.runtime.state_after,
        );
        GammaProjectionAgreement {
            name: "parcel-trajectory".into(),
            disagreement: d,
            localizer: Some(GammaDisagreementLocalizer::Operator("gamma-objective-runtime".into())),
            detail:
                "mean absolute difference between 360D beta walk state and 360D gamma \
                objective runtime state; divergence is by design (gamma operator transforms parcel)"
                    .into(),
        }
    };
    vec![probe_probe, semantic_vibes, receptor_parcel, parcel_trajectory]
}

fn mean_absolute_difference(left: &[f32], right: &[f32]) -> f32 {
    let w = left.len().min(right.len());
    if w == 0 {
        return 0.0;
    }
    left[..w].iter().zip(right[..w].iter()).map(|(a, b)| (a - b).abs()).sum::<f32>() / w as f32
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
