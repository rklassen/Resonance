use crate::ids::{
    ArtifactId, CapabilityId, ContractId, ExecutionId, HashDigest, ModelRef, OverfitPolicyId,
    PayloadId, PhaseToken, PolicyId, ProbeId, RuntimePolicyId, TolerancePolicyId, UtcMinute,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeDeclaration {
    pub id: ProbeId,
    pub name: String,
    pub inputs: Vec<ContractId>,
    pub outputs: Vec<ContractId>,
    pub capabilities: Vec<CapabilityId>,
    pub model: Option<ModelRef>,
    pub prompt: Option<PolicyId>,
    pub preprocessing: PolicyId,
    pub runtime: RuntimePolicyId,
    pub tolerance: TolerancePolicyId,
    pub overfit: OverfitPolicyId,
    pub phase: Option<PhaseToken>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeExecutionRecord {
    pub id: ExecutionId,
    pub probe: ProbeId,
    pub input_artifact: Option<ArtifactId>,
    pub input_payloads: Vec<PayloadId>,
    pub model_hash: Option<HashDigest>,
    pub prompt_hash: Option<HashDigest>,
    pub preprocessing_hash: HashDigest,
    pub runtime: RuntimePolicyId,
    pub tolerance: TolerancePolicyId,
    pub output_payloads: Vec<PayloadId>,
    pub created: UtcMinute,
}
