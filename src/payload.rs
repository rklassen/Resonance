use crate::ids::{
    ArtifactId, ExecutionId, HashDigest, NumericPolicyId, PayloadId, PhaseToken,
    ProvenancePolicyId, UtcMinute, ValueRef,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PayloadRecord {
    pub id: PayloadId,
    pub contract: crate::ids::ContractId,
    pub producer: ExecutionId,
    pub source_artifacts: Vec<ArtifactId>,
    pub source_payloads: Vec<PayloadId>,
    pub value: ValueRef,
    pub hash: Option<HashDigest>,
    pub numeric: Option<NumericPolicyId>,
    pub provenance: ProvenancePolicyId,
    pub phase: Option<PhaseToken>,
    pub created: UtcMinute,
}
