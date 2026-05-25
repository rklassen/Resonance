use crate::ids::{ContractId, FailurePolicyId, GateId, PhaseToken, RequirementId, UtcMinute};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RequirementStatus {
    Active,
    Deferred,
    Removed,
    Superseded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequirementRecord {
    pub id: RequirementId,
    pub name: String,
    pub statement: String,
    pub subject_contracts: Vec<ContractId>,
    pub phases: Vec<PhaseToken>,
    pub verification_gates: Vec<GateId>,
    pub parents: Vec<RequirementId>,
    pub dependencies: Vec<RequirementId>,
    pub failure: FailurePolicyId,
    pub status: RequirementStatus,
    pub created: UtcMinute,
}
