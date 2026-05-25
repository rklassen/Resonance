use crate::ids::{
    ClaimRecordId, GateResultId, OperatorId, OutputId, PhaseToken, TraceId, UtcMinute, ValueRef,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputRecord {
    pub id: OutputId,
    pub name: String,
    pub source_traces: Vec<TraceId>,
    pub included_claims: Vec<ClaimRecordId>,
    pub included_gate_results: Vec<GateResultId>,
    pub export: ValueRef,
    pub generator: OperatorId,
    pub phase: Option<PhaseToken>,
    pub created: UtcMinute,
}
