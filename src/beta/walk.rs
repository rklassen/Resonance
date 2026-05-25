use crate::{
    alpha::AlphaVibes, CapabilityId, ContractId, ExecutionId, NumericPolicyId, OperatorDeclaration,
    OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken, ProvenancePolicyId,
    RuntimePolicyId, SideEffectPolicyId, UtcMinute, ValueRef,
};

use sha2::{Digest, Sha256};

use super::{BetaError, BetaGain, BetaParcelGraph};

const BETA_PHASE: &str = "Β";
const ETA: f32 = 0.12;

#[derive(Clone, Debug, PartialEq)]
pub struct BetaWalk {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub eta: f32,
    pub state_before: Vec<f32>,
    pub laplacian_delta: Vec<f32>,
    pub state_after: Vec<f32>,
}

fn sha256_hex(chunks: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for chunk in chunks {
        hasher.update(chunk);
    }
    format!("{:x}", hasher.finalize())
}

fn short_id(value: &str) -> String {
    value.chars().take(12).collect()
}

pub fn laplacian_runtime(
    graph: &BetaParcelGraph,
    vibes: &AlphaVibes,
    gain: &BetaGain,
) -> Result<BetaWalk, BetaError> {
    let state_before = (0..graph.node_count)
        .map(|index| {
            let left = vibes.signed_12d[index % 12] * 0.55;
            let right = vibes.signed_11d[(index + 5) % 11] * 0.25;
            let hemi = if graph.hemisphere[index] == "L" {
                -0.05
            } else {
                0.05
            };
            (left + right + hemi).clamp(-1.0, 1.0)
        })
        .collect::<Vec<_>>();
    let laplacian_delta = graph.apply_laplacian(&state_before)?;
    if laplacian_delta.len() != graph.node_count {
        return Err(BetaError::new("beta runtime delta width mismatch"));
    }
    if gain.vector.len() != graph.node_count {
        return Err(BetaError::new("beta runtime gain width mismatch"));
    }
    let state_after = state_before
        .iter()
        .zip(laplacian_delta.iter())
        .zip(gain.vector.iter())
        .map(|((state, delta), gain_value)| state - (ETA * delta) + gain_value)
        .collect::<Vec<_>>();
    if state_after.iter().any(|value| !value.is_finite()) {
        return Err(BetaError::new("beta runtime emitted non-finite values"));
    }

    let digest = sha256_hex(&[state_after
        .iter()
        .map(|value| format!("{value:+.6}"))
        .collect::<Vec<_>>()
        .join(",")
        .as_bytes()]);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-beta-laplacian-runtime".into()),
        name: "beta-laplacian-runtime".into(),
        inputs: vec![
            vibes.payload_12d.contract.clone(),
            gain.payload.contract.clone(),
            ContractId("contract.graph.beta-360".into()),
        ],
        outputs: vec![ContractId("contract.payload.beta-state-360".into())],
        capabilities: vec![CapabilityId("capability.graph-walk".into())],
        runtime: RuntimePolicyId("runtime.beta.replayable".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: crate::FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(BETA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-beta-walk-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: Vec::new(),
        input_payloads: vec![
            vibes.payload_12d.id.clone(),
            vibes.payload_11d.id.clone(),
            gain.payload.id.clone(),
        ],
        output_payloads: vec![PayloadId(format!("payload-beta-walk-{}", short_id(&digest)))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605250103),
    };

    Ok(BetaWalk {
        declaration,
        payload: PayloadRecord {
            id: execution.output_payloads[0].clone(),
            contract: ContractId("contract.payload.beta-state-360".into()),
            producer: execution.id.clone(),
            source_artifacts: Vec::new(),
            source_payloads: vec![
                vibes.payload_12d.id.clone(),
                vibes.payload_11d.id.clone(),
                gain.payload.id.clone(),
            ],
            value: ValueRef("inline://beta/walk/state-360".into()),
            hash: Some(crate::HashDigest {
                algorithm: "sha256".into(),
                digest_hex: digest,
            }),
            numeric: Some(NumericPolicyId("numeric.signed-vector.360".into())),
            provenance: ProvenancePolicyId("provenance.explicit".into()),
            phase: Some(PhaseToken(BETA_PHASE.into())),
            created: UtcMinute(202605250103),
        },
        execution,
        eta: ETA,
        state_before,
        laplacian_delta,
        state_after,
    })
}
