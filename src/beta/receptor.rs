use crate::{
    alpha::AlphaVibes, CapabilityId, ContractId, ExecutionId, NumericPolicyId, OperatorDeclaration,
    OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken, ProvenancePolicyId,
    RuntimePolicyId, SideEffectPolicyId, UtcMinute, ValueRef,
};

use sha2::{Digest, Sha256};

use super::{BetaError, BetaPublicFixtures};

const BETA_PHASE: &str = "Β";

#[derive(Clone, Debug, PartialEq)]
pub struct BetaGain {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload: PayloadRecord,
    pub vector: Vec<f32>,
    pub mapping_id: String,
    pub prior_ids: Vec<String>,
    pub terms: Vec<BetaGainTerm>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BetaGainTerm {
    pub prior_id: String,
    pub family: String,
    pub target: String,
    pub coefficient: f32,
    pub evidence_axes: Vec<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct BetaAxisSummary {
    valence: f32,
    arousal: f32,
    dominance: f32,
    warmth: f32,
    tension: f32,
    coherence: f32,
    novelty: f32,
    weight: f32,
    brightness: f32,
    depth: f32,
    motion: f32,
}

pub fn real_receptor_gain(
    fixtures: &BetaPublicFixtures,
    vibes: &AlphaVibes,
) -> Result<BetaGain, BetaError> {
    let axes = BetaAxisSummary::from_vibes(vibes);
    let terms = fixtures
        .priors
        .iter()
        .map(|prior| prior_gain_term(prior, &axes))
        .collect::<Result<Vec<_>, _>>()?;
    let raw = (0..fixtures.parcels.len())
        .map(|node| {
            fixtures
                .priors
                .iter()
                .zip(terms.iter())
                .map(|(prior, term)| prior.values[node] * term.coefficient)
                .sum::<f32>()
        })
        .collect::<Vec<_>>();
    let scale = raw.iter().fold(0.0_f32, |current, value| current.max(value.abs()));
    let vector = if scale == 0.0 {
        raw
    } else {
        raw.iter().map(|value| (value / scale).clamp(-1.0, 1.0)).collect::<Vec<_>>()
    };
    if vector.iter().any(|value| !value.is_finite()) {
        return Err(BetaError::new("beta receptor gain emitted non-finite values"));
    }

    let canonical = vector.iter().map(|value| format!("{value:+.6}")).collect::<Vec<_>>().join(",");
    let term_text = terms
        .iter()
        .map(|term| format!("{}:{}:{:+.6}", term.family, term.target, term.coefficient))
        .collect::<Vec<_>>()
        .join("|");
    let digest = sha256_hex(&[
        canonical.as_bytes(),
        fixtures.atlas.volume_sha256.as_bytes(),
        term_text.as_bytes(),
        fixtures
            .priors
            .iter()
            .map(|prior| prior.value_hash.as_str())
            .collect::<String>()
            .as_bytes(),
    ]);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-beta-receptor-bridge".into()),
        name: "real-beta-receptor-bridge".into(),
        inputs: vec![vibes.payload_12d.contract.clone(), vibes.payload_11d.contract.clone()],
        outputs: vec![ContractId("contract.payload.beta-gain-360".into())],
        capabilities: vec![CapabilityId("capability.prior-gain-bridge".into())],
        runtime: RuntimePolicyId("runtime.beta.replayable".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: crate::FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(BETA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-beta-gain-{}", short_id(&digest))),
        operator: declaration.id.clone(),
        input_artifacts: Vec::new(),
        input_payloads: vec![vibes.payload_12d.id.clone(), vibes.payload_11d.id.clone()],
        output_payloads: vec![PayloadId(format!("payload-beta-gain-{}", short_id(&digest)))],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605250102),
    };

    Ok(BetaGain {
        declaration,
        payload: PayloadRecord {
            id: execution.output_payloads[0].clone(),
            contract: ContractId("contract.payload.beta-gain-360".into()),
            producer: execution.id.clone(),
            source_artifacts: Vec::new(),
            source_payloads: vec![vibes.payload_12d.id.clone(), vibes.payload_11d.id.clone()],
            value: ValueRef("inline://beta/gain/360".into()),
            hash: Some(crate::HashDigest {
                algorithm: "sha256".into(),
                digest_hex: digest,
            }),
            numeric: Some(NumericPolicyId("numeric.signed-vector.360".into())),
            provenance: ProvenancePolicyId("provenance.explicit".into()),
            phase: Some(PhaseToken(BETA_PHASE.into())),
            created: UtcMinute(202605250102),
        },
        execution,
        vector,
        mapping_id: "real-prior-domain-bridge.beta.v2".into(),
        prior_ids: fixtures.priors.iter().map(|prior| prior.id.clone()).collect(),
        terms,
    })
}

fn prior_gain_term(
    prior: &crate::beta::BetaFixturePrior,
    axes: &BetaAxisSummary,
) -> Result<BetaGainTerm, BetaError> {
    let (family, target, coefficient, evidence_axes) =
        match (prior.source.as_str(), prior.desc.as_str()) {
            ("beliveau2017", "dasb") => (
                "serotonin",
                "5-HTT",
                weighted_sum(&[
                    (axes.valence, 0.35),
                    (axes.warmth, 0.25),
                    (axes.coherence, 0.25),
                    (-axes.tension, 0.15),
                ]),
                vec!["valence", "warmth", "coherence", "tension"],
            ),
            ("beliveau2017", "cimbi36") => (
                "serotonin",
                "5-HT2a",
                weighted_sum(&[
                    (axes.arousal, 0.20),
                    (axes.novelty, 0.35),
                    (axes.brightness, 0.25),
                    (axes.motion, 0.20),
                ]),
                vec!["arousal", "novelty", "brightness", "motion"],
            ),
            ("dubois2015", "abp688") => (
                "glutamate",
                "mGluR5",
                weighted_sum(&[
                    (axes.arousal, 0.30),
                    (axes.tension, 0.30),
                    (axes.depth, 0.20),
                    (axes.weight, 0.20),
                ]),
                vec!["arousal", "tension", "depth", "weight"],
            ),
            ("ding2010", "mrb") => (
                "norepinephrine",
                "NET",
                weighted_sum(&[
                    (axes.arousal, 0.35),
                    (axes.dominance, 0.25),
                    (axes.motion, 0.20),
                    (axes.novelty, 0.20),
                ]),
                vec!["arousal", "dominance", "motion", "novelty"],
            ),
            _ => {
                return Err(BetaError::new(format!(
                    "beta receptor bridge does not define semantics for prior {}:{}",
                    prior.source, prior.desc,
                )))
            }
        };

    Ok(BetaGainTerm {
        prior_id: prior.id.clone(),
        family: family.into(),
        target: target.into(),
        coefficient,
        evidence_axes: evidence_axes.into_iter().map(str::to_string).collect(),
    })
}

fn weighted_sum(terms: &[(f32, f32)]) -> f32 {
    terms.iter().map(|(value, weight)| value * weight).sum::<f32>().clamp(-1.0, 1.0)
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

impl BetaAxisSummary {
    fn from_vibes(vibes: &AlphaVibes) -> Self {
        Self {
            valence: vibes.signed_12d[0],
            arousal: vibes.signed_12d[1],
            dominance: vibes.signed_12d[2],
            warmth: vibes.signed_12d[3],
            tension: vibes.signed_12d[4],
            coherence: vibes.signed_12d[5],
            novelty: vibes.signed_12d[6],
            weight: vibes.signed_12d[7],
            brightness: vibes.signed_12d[8],
            depth: vibes.signed_12d[10],
            motion: vibes.signed_12d[11],
        }
    }
}
