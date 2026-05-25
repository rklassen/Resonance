use crate::ids::{
    ArtifactId, BlockedClaimId, ClaimRecordId, ExecutionId, GateResultId, OperatorId, PayloadId,
    PhaseToken, ReplayPolicyId, RunId, SnapEdgeRef, SnapNodeRef, SnapPathRef, TraceId, TraceStepId,
    UtcMinute,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceRecord {
    pub id: TraceId,
    pub run: RunId,
    pub phase: Option<PhaseToken>,
    pub source_artifacts: Vec<ArtifactId>,
    pub operator_executions: Vec<ExecutionId>,
    pub payloads: Vec<PayloadId>,
    pub path: SnapPathRef,
    pub gate_results: Vec<GateResultId>,
    pub claims: Vec<ClaimRecordId>,
    pub blocked_claims: Vec<BlockedClaimId>,
    pub replay: ReplayPolicyId,
    pub created: UtcMinute,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TraceStep {
    pub id: TraceStepId,
    pub trace: TraceId,
    pub operator: OperatorId,
    pub input_payloads: Vec<PayloadId>,
    pub output_payloads: Vec<PayloadId>,
    pub snap_nodes: Vec<SnapNodeRef>,
    pub snap_edges: Vec<SnapEdgeRef>,
    pub gate_results: Vec<GateResultId>,
    pub started: Option<UtcMinute>,
    pub finished: Option<UtcMinute>,
}
