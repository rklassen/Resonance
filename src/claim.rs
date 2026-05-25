use crate::ids::{
    ArtifactId, BlockerId, ClaimRecordId, GateId, GateResultId, PayloadId, PhaseToken, SnapRef,
    TraceId, UtcMinute,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ClaimStatus {
    ObservedFact,
    DerivedClaim,
    BlockedClaim,
    DeferredClaim,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncertaintyRecord {
    pub belief: Option<f32>,
    pub plausibility: Option<f32>,
    pub confidence: Option<f32>,
    pub conflict: Option<f32>,
    pub unsupported_mass: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockerRecord {
    pub blocker: BlockerId,
    pub description: String,
    pub missing_dependency: Option<String>,
    pub failed_gate: Option<GateId>,
    pub prohibited_edge: Option<String>,
    pub required_follow_up: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRecord {
    pub id: ClaimRecordId,
    pub statement: String,
    pub status: ClaimStatus,
    pub phase_scope: Option<PhaseToken>,
    pub support_artifacts: Vec<ArtifactId>,
    pub support_payloads: Vec<PayloadId>,
    pub support_traces: Vec<TraceId>,
    pub support_gate_results: Vec<GateResultId>,
    pub support_snaps: Vec<SnapRef>,
    pub uncertainty: Option<UncertaintyRecord>,
    pub blocker: Option<BlockerRecord>,
    pub created: UtcMinute,
}
