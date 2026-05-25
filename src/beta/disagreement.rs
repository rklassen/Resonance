use crate::{
    alpha::AlphaProbeRun, CapabilityId, ContractId, ExecutionId, NumericPolicyId,
    OperatorDeclaration, OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken,
    ProvenancePolicyId, RuntimePolicyId, SideEffectPolicyId, UtcMinute, ValueRef,
};

use sha2::{Digest, Sha256};

use super::{BetaError, BetaGain, BetaParcelGraph, BetaPublicFixtures, BetaWalk};

const BETA_PHASE: &str = "Β";

#[derive(Clone, Debug, PartialEq)]
pub struct BetaDisagreement {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub probe_disagreement: f32,
    pub receptor_projection_disagreement: f32,
    pub graph_spread: f32,
    pub energy_proxy: f32,
    pub unsupported_edges: usize,
    pub probe_ids: Vec<String>,
    pub prior_ids: Vec<String>,
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

impl BetaDisagreement {
    pub fn measure(
        fixtures: &BetaPublicFixtures,
        embedding: &AlphaProbeRun,
        labels: &AlphaProbeRun,
        gain: &BetaGain,
        graph: &BetaParcelGraph,
        walk: &BetaWalk,
    ) -> Result<Self, BetaError> {
        let probe_width = embedding.values.len().min(labels.values.len());
        let probe_disagreement = embedding.values[..probe_width]
            .iter()
            .zip(labels.values[..probe_width].iter())
            .map(|(left, right)| (left - right).abs())
            .sum::<f32>()
            / probe_width as f32;
        let receptor_projection_disagreement = if fixtures.priors.len() < 2 {
            0.0
        } else {
            (0..fixtures.parcels.len())
                .map(|node| {
                    let mut min = f32::INFINITY;
                    let mut max = f32::NEG_INFINITY;
                    for (prior, term) in fixtures.priors.iter().zip(gain.terms.iter()) {
                        let value = prior.values[node] * term.coefficient;
                        min = min.min(value);
                        max = max.max(value);
                    }
                    max - min
                })
                .sum::<f32>()
                / fixtures.parcels.len() as f32
        };
        let graph_spread = walk.laplacian_delta.iter().map(|value| value.abs()).sum::<f32>()
            / walk.laplacian_delta.len() as f32;
        let energy_proxy = walk.state_after.iter().map(|value| value * value).sum::<f32>()
            / walk.state_after.len() as f32;
        let unsupported_edges = graph
            .adjacency
            .iter()
            .enumerate()
            .map(|(node, neighbors)| {
                neighbors
                    .iter()
                    .filter(|(neighbor, _)| {
                        graph.hemisphere[node] != graph.hemisphere[*neighbor]
                            && graph.parcel_names[node]
                                .trim_start_matches("L_")
                                .trim_start_matches("R_")
                                != graph.parcel_names[*neighbor]
                                    .trim_start_matches("L_")
                                    .trim_start_matches("R_")
                    })
                    .count()
            })
            .sum::<usize>();
        let digest = sha256_hex(&[format!(
            "{probe_disagreement:+.6},{receptor_projection_disagreement:+.6},{graph_spread:+.6},{energy_proxy:+.6},{unsupported_edges}"
        )
        .as_bytes()]);
        let declaration = OperatorDeclaration {
            id: OperatorId("operator-beta-disagreement-width".into()),
            name: "beta-disagreement-width".into(),
            inputs: vec![
                embedding.payload.contract.clone(),
                labels.payload.contract.clone(),
                gain.payload.contract.clone(),
                walk.payload.contract.clone(),
            ],
            outputs: vec![ContractId("contract.payload.beta-disagreement".into())],
            capabilities: vec![CapabilityId("capability.disagreement-width".into())],
            runtime: RuntimePolicyId("runtime.beta.replayable".into()),
            determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
            side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
            failure: crate::FailurePolicyId("failure.stop-and-surface".into()),
            phase: Some(PhaseToken(BETA_PHASE.into())),
        };
        let execution = OperatorExecutionRecord {
            id: ExecutionId(format!("execution-beta-disagreement-{}", short_id(&digest))),
            operator: declaration.id.clone(),
            input_artifacts: Vec::new(),
            input_payloads: vec![
                embedding.payload.id.clone(),
                labels.payload.id.clone(),
                gain.payload.id.clone(),
                walk.payload.id.clone(),
            ],
            output_payloads: vec![PayloadId(format!(
                "payload-beta-disagreement-{}",
                short_id(&digest)
            ))],
            output_gate_results: Vec::new(),
            output_traces: Vec::new(),
            runtime: declaration.runtime.clone(),
            created: UtcMinute(202605250104),
        };

        Ok(Self {
            declaration,
            payload: PayloadRecord {
                id: execution.output_payloads[0].clone(),
                contract: ContractId("contract.payload.beta-disagreement".into()),
                producer: execution.id.clone(),
                source_artifacts: Vec::new(),
                source_payloads: execution.input_payloads.clone(),
                value: ValueRef("inline://beta/disagreement/summary".into()),
                hash: Some(crate::HashDigest {
                    algorithm: "sha256".into(),
                    digest_hex: digest,
                }),
                numeric: Some(NumericPolicyId("numeric.signed-vector.4".into())),
                provenance: ProvenancePolicyId("provenance.explicit".into()),
                phase: Some(PhaseToken(BETA_PHASE.into())),
                created: UtcMinute(202605250104),
            },
            execution,
            probe_disagreement,
            receptor_projection_disagreement,
            graph_spread,
            energy_proxy,
            unsupported_edges,
            probe_ids: vec![embedding.declaration.id.0.clone(), labels.declaration.id.0.clone()],
            prior_ids: gain.terms.iter().map(|term| term.prior_id.clone()).collect(),
        })
    }
}
