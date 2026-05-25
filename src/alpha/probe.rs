use crate::{
    CapabilityId, ContractId, ExecutionId, FailurePolicyId, HashDigest, ModelRef, NumericPolicyId,
    OperatorDeclaration, OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken,
    PolicyId, ProbeDeclaration, ProbeExecutionRecord, ProbeId, ProvenancePolicyId, RuntimePolicyId,
    SideEffectPolicyId, TolerancePolicyId, UtcMinute, ValueRef,
};

use super::artifact::{sha256_hex, short_id, AlphaArtifact};
use super::cache::{AlphaCacheKey, AlphaProbeCache, CacheStatus, CachedProbeRecord};

const ALPHA_PHASE: &str = "Α";

#[derive(Clone, Debug, PartialEq)]
pub struct AlphaProbeRun {
    pub declaration: ProbeDeclaration,
    pub cache_declaration: OperatorDeclaration,
    pub cache_key: AlphaCacheKey,
    pub cache_status: CacheStatus,
    pub cache_execution: OperatorExecutionRecord,
    pub execution: ProbeExecutionRecord,
    pub payload: PayloadRecord,
    pub values: Vec<f32>,
}

pub fn run_embedding_probe(cache: &mut AlphaProbeCache, artifact: &AlphaArtifact) -> AlphaProbeRun {
    run_probe(
        cache,
        artifact,
        probe_spec(
            "probe-alpha-embedding",
            "alpha-embedding-probe",
            "model.alpha.embedding.v1",
            "policy.alpha.embedding.v1",
            "contract.payload.embedding",
            "capability.visual-embedding",
            16,
            UtcMinute(202605240002),
        ),
    )
}

pub fn run_label_probe(cache: &mut AlphaProbeCache, artifact: &AlphaArtifact) -> AlphaProbeRun {
    run_probe(
        cache,
        artifact,
        probe_spec(
            "probe-alpha-affect",
            "alpha-affect-probe",
            "model.alpha.affect.v1",
            "policy.alpha.affect.v1",
            "contract.payload.logits",
            "capability.affective-axis-response",
            12,
            UtcMinute(202605240003),
        ),
    )
}

fn run_probe(
    cache: &mut AlphaProbeCache,
    artifact: &AlphaArtifact,
    spec: ProbeSpec,
) -> AlphaProbeRun {
    let declaration = build_declaration(&spec);
    let key = AlphaCacheKey {
        artifact_hash: artifact.record.hash.digest_hex.clone(),
        model_id: spec.model_id.clone(),
        prompt_id: spec.prompt_id.clone(),
    };

    let lookup = cache.get_or_insert_with(key.clone(), |cache_key| {
        let values = deterministic_values(
            &artifact.normalized_bytes,
            cache_key,
            spec.output_len,
            &spec.probe_id,
        );
        let value_hash = payload_hash(&values);
        let execution = ProbeExecutionRecord {
            id: ExecutionId(format!("execution-{}", short_id(&value_hash.digest_hex))),
            probe: declaration.id.clone(),
            input_artifact: Some(artifact.record.id.clone()),
            input_payloads: Vec::new(),
            model_hash: declaration.model.as_ref().map(|model| model.hash.clone()),
            prompt_hash: Some(hash_string(&spec.prompt_id)),
            preprocessing_hash: hash_string("policy.alpha.preprocessing.v1"),
            runtime: declaration.runtime.clone(),
            tolerance: declaration.tolerance.clone(),
            output_payloads: vec![PayloadId(format!(
                "payload-{}",
                short_id(&value_hash.digest_hex)
            ))],
            created: spec.created,
        };
        let payload = PayloadRecord {
            id: execution.output_payloads[0].clone(),
            contract: ContractId(spec.output_contract.clone()),
            producer: execution.id.clone(),
            source_artifacts: vec![artifact.record.id.clone()],
            source_payloads: Vec::new(),
            value: ValueRef(format!(
                "inline://alpha/cache/{}/{}",
                cache_key.as_string(),
                spec.probe_id
            )),
            hash: Some(value_hash),
            numeric: Some(NumericPolicyId("numeric.signed-vector".into())),
            provenance: ProvenancePolicyId("provenance.explicit".into()),
            phase: Some(PhaseToken(ALPHA_PHASE.into())),
            created: spec.created,
        };
        CachedProbeRecord {
            key: cache_key.clone(),
            execution,
            payload,
            values,
        }
    });

    let cache_declaration = OperatorDeclaration {
        id: OperatorId(format!("operator-cache-{}", spec.probe_id)),
        name: format!("alpha-cache-{}", spec.name),
        inputs: vec![
            ContractId("contract.artifact.alpha".into()),
            ContractId(spec.output_contract.clone()),
        ],
        outputs: vec![ContractId(spec.output_contract.clone())],
        capabilities: vec![CapabilityId("capability.probe-cache".into())],
        runtime: RuntimePolicyId("runtime.alpha.deterministic".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
    };
    let cache_execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-cache-{}", lookup.record.payload.id.0)),
        operator: cache_declaration.id.clone(),
        input_artifacts: vec![artifact.record.id.clone()],
        input_payloads: vec![lookup.record.payload.id.clone()],
        output_payloads: vec![lookup.record.payload.id.clone()],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: cache_declaration.runtime.clone(),
        created: spec.created,
    };

    AlphaProbeRun {
        declaration,
        cache_declaration,
        cache_key: key,
        cache_status: lookup.status,
        cache_execution,
        execution: lookup.record.execution,
        payload: lookup.record.payload,
        values: lookup.record.values,
    }
}

fn deterministic_values(
    bytes: &[u8],
    cache_key: &AlphaCacheKey,
    len: usize,
    probe_id: &str,
) -> Vec<f32> {
    let seed = sha256_hex(&[
        b"probe-alpha-v1",
        probe_id.as_bytes(),
        cache_key.as_string().as_bytes(),
        bytes,
    ]);
    let seed_bytes = seed.as_bytes();
    (0..len)
        .map(|index| {
            let left = seed_bytes[index % seed_bytes.len()] as f32 / 255.0;
            let right = seed_bytes[(index * 7 + 11) % seed_bytes.len()] as f32 / 255.0;
            ((left - right) * 1.8).clamp(-1.0, 1.0)
        })
        .collect()
}

fn payload_hash(values: &[f32]) -> HashDigest {
    HashDigest {
        algorithm: "sha256".into(),
        digest_hex: sha256_hex(&[canonical_vector(values).as_bytes()]),
    }
}

fn canonical_vector(values: &[f32]) -> String {
    values.iter().map(|value| format!("{value:+.6}")).collect::<Vec<_>>().join(",")
}

fn hash_string(value: &str) -> HashDigest {
    HashDigest {
        algorithm: "sha256".into(),
        digest_hex: sha256_hex(&[value.as_bytes()]),
    }
}

fn build_declaration(spec: &ProbeSpec) -> ProbeDeclaration {
    ProbeDeclaration {
        id: ProbeId(spec.probe_id.clone()),
        name: spec.name.clone(),
        inputs: vec![ContractId("contract.artifact.alpha".into())],
        outputs: vec![ContractId(spec.output_contract.clone())],
        capabilities: vec![CapabilityId(spec.capability.clone())],
        model: Some(ModelRef {
            id: spec.model_id.clone(),
            hash: hash_string(&spec.model_id),
            source: Some(format!("registry://models/{}", spec.model_id)),
            license: Some("license://alpha-frozen-mock".into()),
        }),
        prompt: Some(PolicyId(spec.prompt_id.clone())),
        preprocessing: PolicyId("policy.alpha.preprocessing.v1".into()),
        runtime: RuntimePolicyId("runtime.alpha.deterministic".into()),
        tolerance: TolerancePolicyId("tolerance.alpha.exact".into()),
        overfit: crate::OverfitPolicyId("overfit.frozen-baseline".into()),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProbeSpec {
    probe_id: String,
    name: String,
    model_id: String,
    prompt_id: String,
    output_contract: String,
    capability: String,
    output_len: usize,
    created: UtcMinute,
}

fn probe_spec(
    probe_id: &str,
    name: &str,
    model_id: &str,
    prompt_id: &str,
    output_contract: &str,
    capability: &str,
    output_len: usize,
    created: UtcMinute,
) -> ProbeSpec {
    ProbeSpec {
        probe_id: probe_id.into(),
        name: name.into(),
        model_id: model_id.into(),
        prompt_id: prompt_id.into(),
        output_contract: output_contract.into(),
        capability: capability.into(),
        output_len,
        created,
    }
}
