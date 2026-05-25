use crate::{
    CapabilityId, ContractId, ExecutionId, NumericPolicyId, OperatorDeclaration,
    OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken, ProvenancePolicyId,
    RuntimePolicyId, SideEffectPolicyId, UtcMinute, ValueRef,
};

use super::artifact::{sha256_hex, short_id};
use super::graph::AlphaParcelGraph;
use super::receptor::AlphaGain;
use super::vibes::AlphaVibes;
use super::AlphaError;

const ALPHA_PHASE: &str = "Α";
const ETA: f32 = 0.15;

#[derive(Clone, Debug, PartialEq)]
pub struct AlphaWalk {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub eta: f32,
    pub state_before: Vec<f32>,
    pub laplacian_delta: Vec<f32>,
    pub state_after: Vec<f32>,
}

pub fn laplacian_walk(
    graph: &AlphaParcelGraph,
    vibes: &AlphaVibes,
    gain: &AlphaGain,
) -> Result<AlphaWalk, AlphaError> {
    let state_before = (0..graph.node_count)
        .map(|index| {
            let left = vibes.signed_12d[index % 12] * 0.7;
            let right = vibes.signed_11d[(index + 3) % 11] * 0.3;
            (left + right).clamp(-1.0, 1.0)
        })
        .collect::<Vec<_>>();
    let laplacian_delta = graph.apply_laplacian(&state_before)?;
    if laplacian_delta.len() != graph.node_count {
        return Err(AlphaError::new("alpha laplacian walk delta width mismatch"));
    }
    if gain.vector.len() != graph.node_count {
        return Err(AlphaError::new("alpha laplacian walk gain width mismatch"));
    }
    let state_after = state_before
        .iter()
        .zip(laplacian_delta.iter())
        .zip(gain.vector.iter())
        .map(|((state, delta), gain_value)| state - (ETA * delta) + gain_value)
        .collect::<Vec<_>>();

    if state_after.iter().any(|value| !value.is_finite()) {
        return Err(AlphaError::new("alpha laplacian walk emitted non-finite values"));
    }

    let digest = sha256_hex(&[state_after
        .iter()
        .map(|value| format!("{value:+.6}"))
        .collect::<Vec<_>>()
        .join(",")
        .as_bytes()]);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-alpha-laplacian-walk".into()),
        name: "alpha-laplacian-walk".into(),
        inputs: vec![
            vibes.payload_12d.contract.clone(),
            gain.payload.contract.clone(),
            ContractId("contract.graph.alpha-360".into()),
        ],
        outputs: vec![ContractId("contract.payload.walk-state-360".into())],
        capabilities: vec![CapabilityId("capability.graph-walk".into())],
        runtime: RuntimePolicyId("runtime.alpha.deterministic".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: crate::FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-walk-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: Vec::new(),
        input_payloads: vec![
            vibes.payload_12d.id.clone(),
            vibes.payload_11d.id.clone(),
            gain.payload.id.clone(),
        ],
        output_payloads: vec![PayloadId(format!("payload-walk-{}", short_id(&digest)))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605240012),
    };

    Ok(AlphaWalk {
        declaration,
        payload: PayloadRecord {
            id: execution.output_payloads[0].clone(),
            contract: ContractId("contract.payload.walk-state-360".into()),
            producer: execution.id.clone(),
            source_artifacts: Vec::new(),
            source_payloads: vec![
                vibes.payload_12d.id.clone(),
                vibes.payload_11d.id.clone(),
                gain.payload.id.clone(),
            ],
            value: ValueRef("inline://alpha/walk/state-360".into()),
            hash: Some(crate::HashDigest {
                algorithm: "sha256".into(),
                digest_hex: digest,
            }),
            numeric: Some(NumericPolicyId("numeric.signed-vector.360".into())),
            provenance: ProvenancePolicyId("provenance.explicit".into()),
            phase: Some(PhaseToken(ALPHA_PHASE.into())),
            created: UtcMinute(202605240012),
        },
        execution,
        eta: ETA,
        state_before,
        laplacian_delta,
        state_after,
    })
}
