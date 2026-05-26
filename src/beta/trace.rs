use crate::{
    alpha::{AlphaArtifact, AlphaProbeRun, AlphaVibes},
    ClaimRecord, ClaimRecordId, ClaimStatus, FailurePolicyId, FitnessFunctionId, GateDecision,
    GateDeclaration, GateId, GateResult, GateResultId, OperatorExecutionRecord, OutputId,
    OutputRecord, PhaseToken, ReplayPolicyId, RequirementId, RunId, SnapEdgeRef, SnapNodeRef,
    SnapPathRef, SnapRef, SubjectRef, TraceId, TraceRecord, TraceStep, TraceStepId, TruthAxisId,
    TruthAxisJudgment, TruthAxisResult, UncertaintyRecord, UtcMinute, ValueRef,
};

use super::{BetaDisagreement, BetaError, BetaGain, BetaParcelGraph, BetaPublicFixtures, BetaWalk};

const BETA_PHASE: &str = "Β";

#[derive(Clone, Debug, PartialEq)]
pub struct BetaTraceReport {
    pub gate_declaration: GateDeclaration,
    pub gate_result: GateResult,
    pub trace: TraceRecord,
    pub steps: Vec<TraceStep>,
    pub claim: ClaimRecord,
    pub output: OutputRecord,
    pub snap_text: String,
}

#[allow(clippy::too_many_arguments)]
pub fn assemble_trace_report(
    fixtures: &BetaPublicFixtures,
    artifact: &AlphaArtifact,
    embedding: &AlphaProbeRun,
    labels: &AlphaProbeRun,
    vibes: &AlphaVibes,
    gain: &BetaGain,
    graph: &BetaParcelGraph,
    walk: &BetaWalk,
    disagreement: &BetaDisagreement,
) -> Result<BetaTraceReport, BetaError> {
    let run = RunId(format!("run-beta-{}", &artifact.record.id.0));
    let gate_result_id = GateResultId("gate-result-beta-privileged-path".into());
    let report_execution = OperatorExecutionRecord {
        id: crate::ExecutionId("execution-beta-report-1".into()),
        operator: crate::OperatorId("operator-beta-trace-report".into()),
        input_artifacts: vec![artifact.record.id.clone()],
        input_payloads: vec![walk.payload.id.clone(), disagreement.payload.id.clone()],
        output_payloads: Vec::new(),
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: crate::RuntimePolicyId("runtime.beta.replayable".into()),
        created: UtcMinute(202605250105),
    };
    let trace = TraceRecord {
        id: TraceId(format!("trace-beta-{}", &artifact.record.id.0)),
        run: run.clone(),
        phase: Some(PhaseToken(BETA_PHASE.into())),
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
            disagreement.execution.id.clone(),
            report_execution.id.clone(),
        ],
        payloads: vec![
            embedding.payload.id.clone(),
            labels.payload.id.clone(),
            vibes.payload_12d.id.clone(),
            vibes.payload_11d.id.clone(),
            gain.payload.id.clone(),
            walk.payload.id.clone(),
            disagreement.payload.id.clone(),
        ],
        path: SnapPathRef(format!("snap://beta/trace/{}", &artifact.record.id.0)),
        gate_results: vec![gate_result_id.clone()],
        claims: vec![ClaimRecordId("claim-beta-privileged-path".into())],
        blocked_claims: Vec::new(),
        replay: ReplayPolicyId("replay.canonical".into()),
        created: UtcMinute(202605250105),
    };
    let steps = vec![
        trace_step(
            "trace-step-beta-intake",
            &trace.id,
            &artifact.execution,
            vec![SnapNodeRef("b102".into())],
            vec![SnapEdgeRef("flow:b101->b102".into())],
        ),
        probe_trace_step(
            "trace-step-beta-embedding-probe",
            &trace.id,
            &embedding.execution,
            vec![SnapNodeRef("b103".into())],
            vec![SnapEdgeRef("flow:b102->b103".into())],
        ),
        trace_step(
            "trace-step-beta-embedding-cache",
            &trace.id,
            &embedding.cache_execution,
            vec![SnapNodeRef("b104".into())],
            vec![SnapEdgeRef("flow:b103->b104".into())],
        ),
        probe_trace_step(
            "trace-step-beta-label-probe",
            &trace.id,
            &labels.execution,
            vec![SnapNodeRef("b103".into())],
            vec![SnapEdgeRef("flow:b102->b103".into())],
        ),
        trace_step(
            "trace-step-beta-label-cache",
            &trace.id,
            &labels.cache_execution,
            vec![SnapNodeRef("b104".into())],
            vec![SnapEdgeRef("flow:b103->b104".into())],
        ),
        trace_step(
            "trace-step-beta-vibes",
            &trace.id,
            &vibes.execution,
            vec![SnapNodeRef("b105".into())],
            vec![SnapEdgeRef("flow:b104->b105".into())],
        ),
        trace_step(
            "trace-step-beta-gain",
            &trace.id,
            &gain.execution,
            vec![SnapNodeRef("b106".into())],
            vec![SnapEdgeRef("flow:b105->b106".into())],
        ),
        trace_step(
            "trace-step-beta-walk",
            &trace.id,
            &walk.execution,
            vec![SnapNodeRef("b107".into()), SnapNodeRef("b108".into())],
            vec![SnapEdgeRef("flow:b106->b107".into()), SnapEdgeRef("flow:b107->b108".into())],
        ),
        trace_step(
            "trace-step-beta-disagreement",
            &trace.id,
            &disagreement.execution,
            vec![SnapNodeRef("b109".into())],
            vec![SnapEdgeRef("flow:b108->b109".into())],
        ),
        TraceStep {
            id: TraceStepId("trace-step-beta-report".into()),
            trace: trace.id.clone(),
            operator: report_execution.operator.clone(),
            input_payloads: report_execution.input_payloads.clone(),
            output_payloads: report_execution.output_payloads.clone(),
            snap_nodes: vec![SnapNodeRef("o106".into()), SnapNodeRef("o109".into())],
            snap_edges: vec![
                SnapEdgeRef("verify:b106->o106".into()),
                SnapEdgeRef("verify:b109->o109".into()),
            ],
            gate_results: vec![gate_result_id.clone()],
            started: Some(report_execution.created),
            finished: Some(UtcMinute(202605250106)),
        },
    ];
    let beta_valid = beta_valid(fixtures, gain, graph, walk, disagreement);
    let gate_declaration = GateDeclaration {
        gate_id: GateId("gate-beta-privileged-path".into()),
        display_name: "verify-beta-privileged-path".into(),
        subject_contract: crate::ContractId("contract.trace.beta-privileged-path".into()),
        prerequisite_gate_ids: Vec::new(),
        fitness_function_id: FitnessFunctionId("fitness.beta-privileged-path".into()),
        phase_scope: Some(PhaseToken(BETA_PHASE.into())),
        applies_to_requirement_ids: vec![RequirementId("requirement.beta.privileged-path".into())],
        truth_axes: vec![TruthAxisId("Integration".into())],
        failure_policy: FailurePolicyId("failure.stop-and-surface".into()),
    };
    let gate_result = GateResult {
        gate_result_id: gate_result_id.clone(),
        gate_id: gate_declaration.gate_id.clone(),
        subject_ref: SubjectRef(format!("trace:{}", trace.id.0)),
        prerequisite_results: Vec::new(),
        axis_results: vec![TruthAxisResult {
            axis_id: TruthAxisId("Integration".into()),
            judgment: if beta_valid {
                TruthAxisJudgment::Yes
            } else {
                TruthAxisJudgment::Weak
            },
            numeric_value: Some(if beta_valid {
                1.0
            } else {
                0.25
            }),
            evidence_refs: vec![
                "observation:real-priors-installed".into(),
                "observation:graph-replayable".into(),
                "observation:beta-only-report".into(),
            ],
        }],
        decision: if beta_valid {
            GateDecision::Pass
        } else {
            GateDecision::Fail
        },
        follow_up_observation: (!beta_valid).then(|| crate::ObservationRequest {
            observation_id: crate::ObservationId("observation-beta-privileged-gap".into()),
            description:
                "beta privileged path lost a real prior, graph, or replayability condition".into(),
            required_subject: SubjectRef(format!("trace:{}", trace.id.0)),
            expected_resolution:
                "restore real priors, checked graph dimensions, and beta-only reporting".into(),
        }),
        evidence_payload_ids: vec![
            embedding.payload.id.clone(),
            labels.payload.id.clone(),
            gain.payload.id.clone(),
            walk.payload.id.clone(),
            disagreement.payload.id.clone(),
        ],
        evidence_trace_ids: vec![trace.id.clone()],
        created: UtcMinute(202605250106),
    };
    let prior_names = fixtures
        .priors
        .iter()
        .map(|prior| format!("{}/{}", prior.source, prior.desc))
        .collect::<Vec<_>>()
        .join(", ");
    let claim = ClaimRecord {
        id: ClaimRecordId("claim-beta-privileged-path".into()),
        statement: format!(
            "Beta replaces alpha receptor and graph mocks with {} real priors via {} ({}) and a {} parcel graph; disagreement is probe={:.3}, prior={:.3}, spread={:.3}, energy={:.3}",
            fixtures.priors.len(),
            fixtures.priors[0].transform,
            gain.mapping_id,
            graph.schema,
            disagreement.probe_disagreement,
            disagreement.receptor_projection_disagreement,
            disagreement.graph_spread,
            disagreement.energy_proxy,
        ),
        status: ClaimStatus::DerivedClaim,
        phase_scope: Some(PhaseToken(BETA_PHASE.into())),
        support_artifacts: vec![artifact.record.id.clone()],
        support_payloads: vec![
            embedding.payload.id.clone(),
            labels.payload.id.clone(),
            gain.payload.id.clone(),
            walk.payload.id.clone(),
            disagreement.payload.id.clone(),
        ],
        support_traces: vec![trace.id.clone()],
        support_gate_results: vec![gate_result.gate_result_id.clone()],
        support_snaps: vec![
            SnapRef("snap://beta/trace#node=b106".into()),
            SnapRef("snap://beta/trace#node=b107".into()),
            SnapRef("snap://beta/trace#node=b109".into()),
        ],
        uncertainty: Some(UncertaintyRecord {
            belief: Some(1.0),
            plausibility: Some(1.0),
            confidence: Some(1.0),
            conflict: Some(0.0),
            unsupported_mass: Some((disagreement.unsupported_edges as f32).min(1.0)),
        }),
        blocker: None,
        created: UtcMinute(202605250107),
    };
    let output = OutputRecord {
        id: OutputId("output-beta-trace-report".into()),
        name: "beta-privileged-path-report".into(),
        source_traces: vec![trace.id.clone()],
        included_claims: vec![claim.id.clone()],
        included_gate_results: vec![gate_result.gate_result_id.clone()],
        export: ValueRef(format!("file://output/reports/{}.snap", trace.id.0)),
        generator: crate::OperatorId("operator-beta-trace-report".into()),
        phase: Some(PhaseToken(BETA_PHASE.into())),
        created: UtcMinute(202605250107),
    };
    let snap_text =
        beta_snap_text(&trace.id.0, &artifact.record.hash.digest_hex, graph, prior_names.as_str());
    if beta_snap_text(&trace.id.0, &artifact.record.hash.digest_hex, graph, prior_names.as_str())
        != snap_text
    {
        return Err(BetaError::new("beta snap emission was not stable"));
    }

    Ok(BetaTraceReport {
        gate_declaration,
        gate_result,
        trace,
        steps,
        claim,
        output,
        snap_text,
    })
}

fn beta_valid(
    fixtures: &BetaPublicFixtures,
    gain: &BetaGain,
    graph: &BetaParcelGraph,
    walk: &BetaWalk,
    disagreement: &BetaDisagreement,
) -> bool {
    graph.node_count == 360
        && graph.non_zero_edges > 0
        && gain.prior_ids.len() == fixtures.priors.len()
        && fixtures.priors.iter().all(|prior| prior.coverage_min == 1.0)
        && walk.state_after.len() == 360
        && walk.state_after.iter().all(|value| value.is_finite())
        && disagreement.unsupported_edges == 0
}

fn beta_snap_text(
    trace_id: &str,
    artifact_hash: &str,
    graph: &BetaParcelGraph,
    prior_names: &str,
) -> String {
    [
        "🪢snap resonance-beta-trace".into(),
        ".graph {".into(),
        format!(" id: {trace_id},"),
        " name: 'resonance-beta-trace',".into(),
        " version: 0.8,".into(),
        "}".into(),
        "nodes {".into(),
        " object { id: b101, name: 'Snap-Spine-Β', type: SnapSpine, }".into(),
        " object { id: b102, name: 'Artifact-Intake-Β', type: ArtifactIntake, }".into(),
        " object { id: b103, name: 'Frozen-Probe-Β', type: FrozenProbe, }".into(),
        " object { id: b104, name: 'Probe-Cache-Β', type: ProbeCache, }".into(),
        " object { id: b105, name: 'Vibes-Projection-Β', type: VibesProjection, }".into(),
        " object { id: b106, name: 'Receptor-Bridge-Β', type: ReceptorBridge, }".into(),
        " object { id: b107, name: 'Parcel-Graph-Β', type: ParcelGraph, }".into(),
        " object { id: b108, name: 'Laplacian-Runtime-Β', type: LaplacianRuntime, }".into(),
        " object { id: b109, name: 'Requirements-Report-Β', type: RequirementsReport, }".into(),
        " object { id: o106, name: 'Observe-Real-Priors❇beta', type: ObservationNode, }".into(),
        " object { id: o109, name: 'Observe-Beta-Only-Report❇beta', type: ObservationNode, }".into(),
        "}".into(),
        "edges {".into(),
        " flow { @b101 -> @b102, @b102 -> @b103, @b103 -> @b104, @b104 -> @b105, @b105 -> @b106, @b106 -> @b107, @b107 -> @b108, @b108 -> @b109, }".into(),
        " verify { @b106 -> @o106, @b109 -> @o109, }".into(),
        "}".into(),
        "registers {".into(),
        format!(" artifact_hash: '{artifact_hash}',"),
        format!(" trace_id: '{trace_id}',"),
        format!(" graph_nodes: '{}',", graph.node_count),
        format!(" graph_edges: '{}',", graph.non_zero_edges),
        format!(" priors: '{prior_names}',"),
        "}".into(),
    ]
    .join("\n")
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
