use crate::{
    BlockerId, ClaimRecord, ClaimRecordId, ClaimStatus, FailurePolicyId, FitnessFunctionId,
    GateDecision, GateDeclaration, GateId, GateResult, GateResultId, OperatorExecutionRecord,
    OutputId, OutputRecord, PhaseToken, ReplayPolicyId, RequirementId, RunId, SnapEdgeRef,
    SnapNodeRef, SnapPathRef, SnapRef, SubjectRef, TraceId, TraceRecord, TraceStep, TraceStepId,
    TruthAxisId, TruthAxisJudgment, TruthAxisResult, UncertaintyRecord, UtcMinute, ValueRef,
};

use super::artifact::AlphaArtifact;
use super::graph::AlphaParcelGraph;
use super::probe::AlphaProbeRun;
use super::receptor::AlphaGain;
use super::snap::{alpha_trace_document, AlphaSnapDocument};
use super::totality::totality_complete;
use super::vibes::AlphaVibes;
use super::walk::AlphaWalk;
use super::AlphaError;

const ALPHA_PHASE: &str = "Α";

#[derive(Clone, Debug, PartialEq)]
pub struct AlphaTraceReport {
    pub gate_declaration: GateDeclaration,
    pub gate_result: GateResult,
    pub trace: TraceRecord,
    pub steps: Vec<TraceStep>,
    pub claim: ClaimRecord,
    pub output: OutputRecord,
    pub snap: AlphaSnapDocument,
    pub snap_text: String,
}

pub fn assemble_trace_report(
    artifact: &AlphaArtifact,
    embedding: &AlphaProbeRun,
    labels: &AlphaProbeRun,
    vibes: &AlphaVibes,
    gain: &AlphaGain,
    graph: &AlphaParcelGraph,
    walk: &AlphaWalk,
) -> Result<AlphaTraceReport, AlphaError> {
    let run = RunId(format!("run-{}", &artifact.record.id.0));
    let gate_result_id = GateResultId("gate-result-alpha-totality".into());
    let report_execution = OperatorExecutionRecord {
        id: crate::ExecutionId("execution-alpha-report-1".into()),
        operator: crate::OperatorId("operator-alpha-trace-report".into()),
        input_artifacts: vec![artifact.record.id.clone()],
        input_payloads: vec![walk.payload.id.clone()],
        output_payloads: Vec::new(),
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: crate::RuntimePolicyId("runtime.alpha.deterministic".into()),
        created: UtcMinute(202605240013),
    };
    let trace = TraceRecord {
        id: TraceId(format!("trace-{}", &artifact.record.id.0)),
        run: run.clone(),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
        source_artifacts: vec![artifact.record.id.clone()],
        operator_executions: vec![
            artifact.execution.id.clone(),
            embedding.execution.id.clone(),
            embedding.cache_execution.id.clone(),
            labels.execution.id.clone(),
            labels.cache_execution.id.clone(),
            vibes.execution.id.clone(),
            gain.execution.id.clone(),
            walk.execution.id.clone(),
            report_execution.id.clone(),
        ],
        payloads: vec![
            embedding.payload.id.clone(),
            labels.payload.id.clone(),
            vibes.payload_12d.id.clone(),
            vibes.payload_11d.id.clone(),
            gain.payload.id.clone(),
            walk.payload.id.clone(),
        ],
        path: SnapPathRef(format!("snap://alpha/trace/{}", &artifact.record.id.0)),
        gate_results: vec![gate_result_id.clone()],
        claims: vec![ClaimRecordId("claim-alpha-totality".into())],
        blocked_claims: Vec::new(),
        replay: ReplayPolicyId("replay.canonical".into()),
        created: UtcMinute(202605240013),
    };

    let steps = vec![
        trace_step(
            "trace-step-intake",
            &trace.id,
            &artifact.execution,
            vec![SnapNodeRef("a102".into())],
            vec![SnapEdgeRef("flow:a101->a102".into())],
        ),
        probe_trace_step(
            "trace-step-embedding-probe",
            &trace.id,
            &embedding.execution,
            vec![SnapNodeRef("a103".into())],
            vec![SnapEdgeRef("flow:a102->a103".into())],
        ),
        trace_step(
            "trace-step-embedding-cache",
            &trace.id,
            &embedding.cache_execution,
            vec![SnapNodeRef("a104".into())],
            vec![SnapEdgeRef("flow:a103->a104".into())],
        ),
        probe_trace_step(
            "trace-step-label-probe",
            &trace.id,
            &labels.execution,
            vec![SnapNodeRef("a103".into())],
            vec![SnapEdgeRef("flow:a102->a103".into())],
        ),
        trace_step(
            "trace-step-label-cache",
            &trace.id,
            &labels.cache_execution,
            vec![SnapNodeRef("a104".into())],
            vec![SnapEdgeRef("flow:a103->a104".into())],
        ),
        trace_step(
            "trace-step-vibes",
            &trace.id,
            &vibes.execution,
            vec![SnapNodeRef("a105".into())],
            vec![SnapEdgeRef("flow:a104->a105".into())],
        ),
        trace_step(
            "trace-step-gain",
            &trace.id,
            &gain.execution,
            vec![SnapNodeRef("a106".into()), SnapNodeRef("a107".into())],
            vec![SnapEdgeRef("flow:a105->a106".into()), SnapEdgeRef("flow:a106->a107".into())],
        ),
        trace_step(
            "trace-step-walk",
            &trace.id,
            &walk.execution,
            vec![SnapNodeRef("a108".into())],
            vec![SnapEdgeRef("flow:a107->a108".into())],
        ),
        TraceStep {
            id: TraceStepId("trace-step-report".into()),
            trace: trace.id.clone(),
            operator: report_execution.operator.clone(),
            input_payloads: report_execution.input_payloads.clone(),
            output_payloads: report_execution.output_payloads.clone(),
            snap_nodes: vec![SnapNodeRef("a109".into()), SnapNodeRef("o109".into())],
            snap_edges: vec![SnapEdgeRef("verify:a109->o109".into())],
            gate_results: vec![gate_result_id.clone()],
            started: Some(report_execution.created),
            finished: Some(UtcMinute(202605240014)),
        },
    ];
    let snap =
        alpha_trace_document(&trace.id.0, &artifact.record.hash.digest_hex, graph.node_count);
    let totality_complete = totality_complete(
        artifact,
        embedding,
        labels,
        vibes,
        gain,
        graph,
        walk,
        &report_execution,
        &trace,
        &steps,
        &snap,
    );

    let gate_declaration = GateDeclaration {
        gate_id: GateId("gate-alpha-totality".into()),
        display_name: "verify-alpha-totality-integration".into(),
        subject_contract: crate::ContractId("contract.trace.alpha-totality".into()),
        prerequisite_gate_ids: Vec::new(),
        fitness_function_id: FitnessFunctionId("fitness.alpha-totality".into()),
        phase_scope: Some(PhaseToken(ALPHA_PHASE.into())),
        applies_to_requirement_ids: vec![RequirementId("requirement.alpha.totality".into())],
        truth_axes: vec![TruthAxisId("Integration".into())],
        failure_policy: FailurePolicyId("failure.stop-and-surface".into()),
    };
    let gate_result = GateResult {
        gate_result_id: gate_result_id.clone(),
        gate_id: gate_declaration.gate_id.clone(),
        subject_ref: SubjectRef(format!("trace:{}", trace.id.0)),
        prerequisite_results: Vec::new(),
        axis_results: vec![axis_result("Integration", totality_complete)],
        decision: if totality_complete {
            GateDecision::Pass
        } else {
            GateDecision::Fail
        },
        follow_up_observation: (!totality_complete).then(|| crate::ObservationRequest {
            observation_id: crate::ObservationId("observation-alpha-totality-gap".into()),
            description: "record missing alpha intake, cache, or report provenance".into(),
            required_subject: SubjectRef(format!("trace:{}", trace.id.0)),
            expected_resolution: "record every alpha module in the totality trace".into(),
        }),
        evidence_payload_ids: vec![
            embedding.payload.id.clone(),
            labels.payload.id.clone(),
            vibes.payload_12d.id.clone(),
            gain.payload.id.clone(),
            walk.payload.id.clone(),
        ],
        evidence_trace_ids: vec![trace.id.clone()],
        created: UtcMinute(202605240014),
    };

    let claim = ClaimRecord {
        id: ClaimRecordId("claim-alpha-totality".into()),
        statement: format!(
            "Alpha totality trace covers intake, probes, cache, projection, mock gain, walk, and reporting for the declared input; cache statuses were embedding={}, labels={}",
            cache_status_name(&embedding.cache_status),
            cache_status_name(&labels.cache_status),
        ),
        status: ClaimStatus::DerivedClaim,
        phase_scope: Some(PhaseToken(ALPHA_PHASE.into())),
        support_artifacts: vec![artifact.record.id.clone()],
        support_payloads: vec![
            embedding.payload.id.clone(),
            labels.payload.id.clone(),
            vibes.payload_12d.id.clone(),
            gain.payload.id.clone(),
            walk.payload.id.clone(),
        ],
        support_traces: vec![trace.id.clone()],
        support_gate_results: vec![gate_result.gate_result_id.clone()],
        support_snaps: vec![
            SnapRef("snap://alpha/trace#node=o101".into()),
            SnapRef("snap://alpha/trace#node=o103".into()),
            SnapRef("snap://alpha/trace#node=o105".into()),
            SnapRef("snap://alpha/trace#node=o109".into()),
        ],
        uncertainty: Some(UncertaintyRecord {
            belief: Some(1.0),
            plausibility: Some(1.0),
            confidence: Some(1.0),
            conflict: Some(0.0),
            unsupported_mass: Some(0.0),
        }),
        blocker: Some(crate::BlockerRecord {
            blocker: BlockerId("alpha-mocks-phase-scoped".into()),
            description:
                "Alpha uses explicit mock receptor gain and mock parcel graph only for shape proof."
                    .into(),
            missing_dependency: Some("beta real priors and parcel graph".into()),
            failed_gate: None,
            prohibited_edge: None,
            required_follow_up: Some("replace mocks in beta".into()),
        }),
        created: UtcMinute(202605240015),
    };
    let output = OutputRecord {
        id: OutputId("output-alpha-trace-report".into()),
        name: "alpha-totality-report".into(),
        source_traces: vec![trace.id.clone()],
        included_claims: vec![claim.id.clone()],
        included_gate_results: vec![gate_result.gate_result_id.clone()],
        export: ValueRef(format!("file://output/reports/{}.snap", trace.id.0)),
        generator: crate::OperatorId("operator-alpha-trace-report".into()),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
        created: UtcMinute(202605240016),
    };

    let snap_text = snap.to_text();
    let reparsed = AlphaSnapDocument::parse(&snap_text)?;
    if reparsed.to_text() != snap_text {
        return Err(AlphaError::new("alpha snap roundtrip was not stable"));
    }

    Ok(AlphaTraceReport {
        gate_declaration,
        gate_result,
        trace,
        steps,
        claim,
        output,
        snap,
        snap_text,
    })
}

fn axis_result(name: &str, complete: bool) -> TruthAxisResult {
    TruthAxisResult {
        axis_id: TruthAxisId(name.into()),
        judgment: if complete {
            TruthAxisJudgment::Yes
        } else {
            TruthAxisJudgment::Weak
        },
        numeric_value: Some(if complete {
            1.0
        } else {
            0.25
        }),
        evidence_refs: vec![format!("observation:{name}"), "observation:totality-trace".into()],
    }
}

fn cache_status_name(status: &crate::CacheStatus) -> &'static str {
    match status {
        crate::CacheStatus::Hit => "hit",
        crate::CacheStatus::Miss => "miss",
    }
}

fn probe_trace_step(
    id: &str,
    trace_id: &TraceId,
    execution: &crate::ProbeExecutionRecord,
    snap_nodes: Vec<SnapNodeRef>,
    snap_edges: Vec<SnapEdgeRef>,
) -> TraceStep {
    TraceStep {
        id: TraceStepId(id.into()),
        trace: trace_id.clone(),
        operator: crate::OperatorId(execution.probe.0.clone()),
        input_payloads: execution.input_payloads.clone(),
        output_payloads: execution.output_payloads.clone(),
        snap_nodes,
        snap_edges,
        gate_results: Vec::new(),
        started: Some(execution.created),
        finished: Some(execution.created),
    }
}

fn trace_step(
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
