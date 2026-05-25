use crate::{
    CapabilityId, ContractId, ExecutionId, NumericPolicyId, OperatorDeclaration,
    OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken, ProvenancePolicyId,
    RuntimePolicyId, SideEffectPolicyId, UtcMinute, ValueRef,
};

use super::artifact::{sha256_hex, short_id};
use super::vibes::AlphaVibes;
use super::AlphaError;

const ALPHA_PHASE: &str = "Α";
pub const PARCEL_COUNT: usize = 360;

#[derive(Clone, Debug, PartialEq)]
pub struct AlphaGain {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub vector: Vec<f32>,
    pub mapping_id: String,
}

pub fn mock_receptor_gain(vibes: &AlphaVibes) -> Result<AlphaGain, AlphaError> {
    let vector = (0..PARCEL_COUNT)
        .map(|index| {
            let base = vibes.signed_12d[index % 12] * 0.65;
            let folded = vibes.signed_11d[(index * 5) % 11] * 0.35;
            let offset = ((index % 9) as f32 - 4.0) / 40.0;
            (base + folded + offset).clamp(-1.0, 1.0)
        })
        .collect::<Vec<_>>();

    if vector.iter().any(|value| !value.is_finite()) {
        return Err(AlphaError::new("alpha receptor gain emitted non-finite values"));
    }

    let digest = sha256_hex(&[vector
        .iter()
        .map(|value| format!("{value:+.6}"))
        .collect::<Vec<_>>()
        .join(",")
        .as_bytes()]);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-alpha-receptor-gain".into()),
        name: "mock-alpha-receptor-gain".into(),
        inputs: vec![vibes.payload_12d.contract.clone(), vibes.payload_11d.contract.clone()],
        outputs: vec![ContractId("contract.payload.gain-360".into())],
        capabilities: vec![CapabilityId("capability.prior-gain-bridge".into())],
        runtime: RuntimePolicyId("runtime.alpha.deterministic".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: crate::FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-gain-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: Vec::new(),
        input_payloads: vec![vibes.payload_12d.id.clone(), vibes.payload_11d.id.clone()],
        output_payloads: vec![PayloadId(format!("payload-gain-{}", short_id(&digest)))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605240011),
    };

    Ok(AlphaGain {
        declaration,
        payload: PayloadRecord {
            id: execution.output_payloads[0].clone(),
            contract: ContractId("contract.payload.gain-360".into()),
            producer: execution.id.clone(),
            source_artifacts: Vec::new(),
            source_payloads: vec![vibes.payload_12d.id.clone(), vibes.payload_11d.id.clone()],
            value: ValueRef("inline://alpha/gain/360".into()),
            hash: Some(crate::HashDigest {
                algorithm: "sha256".into(),
                digest_hex: digest.clone(),
            }),
            numeric: Some(NumericPolicyId("numeric.signed-vector.360".into())),
            provenance: ProvenancePolicyId("provenance.explicit".into()),
            phase: Some(PhaseToken(ALPHA_PHASE.into())),
            created: UtcMinute(202605240011),
        },
        execution,
        vector,
        mapping_id: "mock.receptor.gain.alpha.v1".into(),
    })
}
