#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockedClaimId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockerId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClaimRecordId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContractId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeterminismPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExecutionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FailurePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FitnessFunctionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GateId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GateResultId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NumericPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObservationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OperatorId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutputId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OverfitPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PayloadId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhaseToken(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProbeId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProvenancePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReplayPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequirementId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SideEffectPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapEdgeRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapNodeRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapPathRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StateId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubjectRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TolerancePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraceStepId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TruthAxisId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ValueRef(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UtcMinute(pub i64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashDigest {
    pub algorithm: String,
    pub digest_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelRef {
    pub id: String,
    pub hash: HashDigest,
    pub source: Option<String>,
    pub license: Option<String>,
}
