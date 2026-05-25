use crate::ids::{
    ContractId, FailurePolicyId, FitnessFunctionId, GateId, GateResultId, ObservationId, PayloadId,
    PhaseToken, RequirementId, SubjectRef, TraceId, TruthAxisId, UtcMinute,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GateDecision {
    Pass,
    Fail,
    Blocked,
    Deferred,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TruthAxisJudgment {
    Yes,
    Mostly,
    Mixed,
    Weak,
    Violation,
    NotApplicable,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TruthAxisResult {
    pub axis_id: TruthAxisId,
    pub judgment: TruthAxisJudgment,
    pub numeric_value: Option<f32>,
    pub evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservationRequest {
    pub observation_id: ObservationId,
    pub description: String,
    pub required_subject: SubjectRef,
    pub expected_resolution: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateDeclaration {
    pub gate_id: GateId,
    pub display_name: String,
    pub subject_contract: ContractId,
    pub prerequisite_gate_ids: Vec<GateId>,
    pub fitness_function_id: FitnessFunctionId,
    pub phase_scope: Option<PhaseToken>,
    pub applies_to_requirement_ids: Vec<RequirementId>,
    pub truth_axes: Vec<TruthAxisId>,
    pub failure_policy: FailurePolicyId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GateResult {
    pub gate_result_id: GateResultId,
    pub gate_id: GateId,
    pub subject_ref: SubjectRef,
    pub prerequisite_results: Vec<GateResultId>,
    pub axis_results: Vec<TruthAxisResult>,
    pub decision: GateDecision,
    pub follow_up_observation: Option<ObservationRequest>,
    pub evidence_payload_ids: Vec<PayloadId>,
    pub evidence_trace_ids: Vec<TraceId>,
    pub created: UtcMinute,
}
