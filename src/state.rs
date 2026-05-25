use crate::ids::{
    GateResultId, OperatorId, PayloadId, PhaseToken, StateId, SubjectRef, TraceId, UtcMinute,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StateLabel(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MutationVerb {
    Promote,
    Wrap,
    Replace,
    Extend,
    StressTest,
    Transform,
    Remove,
    Defer,
    Block,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateRecord {
    pub id: StateId,
    pub subject: SubjectRef,
    pub label: StateLabel,
    pub phase: Option<PhaseToken>,
    pub entered_by: Option<OperatorId>,
    pub exited_by: Option<OperatorId>,
    pub allowed_mutations: Vec<MutationVerb>,
    pub evidence_payloads: Vec<PayloadId>,
    pub evidence_gate_results: Vec<GateResultId>,
    pub evidence_traces: Vec<TraceId>,
    pub created: UtcMinute,
}
