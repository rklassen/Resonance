use crate::ids::{
    ArtifactId, CapabilityId, ContractId, DeterminismPolicyId, ExecutionId, FailurePolicyId,
    GateResultId, OperatorId, PayloadId, PhaseToken, RuntimePolicyId, SideEffectPolicyId, TraceId,
    UtcMinute,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorDeclaration {
    pub id: OperatorId,
    pub name: String,
    pub inputs: Vec<ContractId>,
    pub outputs: Vec<ContractId>,
    pub capabilities: Vec<CapabilityId>,
    pub runtime: RuntimePolicyId,
    pub determinism: DeterminismPolicyId,
    pub side_effects: SideEffectPolicyId,
    pub failure: FailurePolicyId,
    pub phase: Option<PhaseToken>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorExecutionRecord {
    pub id: ExecutionId,
    pub operator: OperatorId,
    pub input_artifacts: Vec<ArtifactId>,
    pub input_payloads: Vec<PayloadId>,
    pub output_payloads: Vec<PayloadId>,
    pub output_gate_results: Vec<GateResultId>,
    pub output_traces: Vec<TraceId>,
    pub runtime: RuntimePolicyId,
    pub created: UtcMinute,
}
