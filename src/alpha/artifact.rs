use sha2::{Digest, Sha256};

use crate::{
    ArtifactId, ArtifactRecord, CapabilityId, ContractId, ExecutionId, FailurePolicyId, HashDigest,
    MetadataEntry, OperatorDeclaration, OperatorExecutionRecord, OperatorId, PhaseToken,
    RuntimePolicyId, SideEffectPolicyId, UtcMinute, ValueRef,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaArtifact {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub record: ArtifactRecord,
    pub media_type: String,
    pub normalized_bytes: Vec<u8>,
    pub normalized_text: Option<String>,
}

const ALPHA_PHASE: &str = "Α";

pub fn load_text(source: &str, text: &str) -> AlphaArtifact {
    let normalized_text = normalize_text(text);
    let media_type = "text/plain;charset=utf-8".to_string();
    let normalized_bytes = normalized_text.as_bytes().to_vec();
    build_artifact(source, media_type, normalized_bytes, Some(normalized_text))
}

pub fn load_image(source: &str, media_type: &str, bytes: &[u8]) -> AlphaArtifact {
    build_artifact(source, normalize_media_type(media_type), bytes.to_vec(), None)
}

pub fn hash_artifact(media_type: &str, normalized_bytes: &[u8]) -> HashDigest {
    HashDigest {
        algorithm: "sha256".into(),
        digest_hex: sha256_hex(&[b"artifact-alpha-v1", media_type.as_bytes(), normalized_bytes]),
    }
}

pub fn normalize_metadata(entries: Vec<(String, String)>) -> Vec<MetadataEntry> {
    let mut metadata = entries
        .into_iter()
        .map(|(key, value)| MetadataEntry {
            key: key.trim().to_ascii_lowercase(),
            value: value.trim().to_string(),
        })
        .collect::<Vec<_>>();
    metadata.sort_by(|left, right| left.key.cmp(&right.key).then(left.value.cmp(&right.value)));
    metadata
}

pub(crate) fn sha256_hex(parts: &[&[u8]]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part);
    }
    hex_encode(&hasher.finalize())
}

pub(crate) fn short_id(hex: &str) -> String {
    hex.chars().take(12).collect()
}

fn build_artifact(
    source: &str,
    media_type: String,
    normalized_bytes: Vec<u8>,
    normalized_text: Option<String>,
) -> AlphaArtifact {
    let hash = hash_artifact(&media_type, &normalized_bytes);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-alpha-artifact-intake".into()),
        name: "alpha-artifact-intake".into(),
        inputs: vec![ContractId("contract.artifact.source".into())],
        outputs: vec![ContractId("contract.artifact.alpha".into())],
        capabilities: vec![CapabilityId("capability.artifact-intake".into())],
        runtime: RuntimePolicyId("runtime.alpha.deterministic".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-intake-{}", short_id(&hash.digest_hex))),
        operator: declaration.id.clone(),
        input_artifacts: Vec::new(),
        input_payloads: Vec::new(),
        output_payloads: Vec::new(),
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605240001),
    };
    let byte_length = normalized_bytes.len().to_string();
    let metadata = normalize_metadata(match normalized_text.as_ref() {
        Some(text) => vec![
            ("byte_length".into(), byte_length),
            ("char_length".into(), text.chars().count().to_string()),
            ("media_type".into(), media_type.clone()),
            ("normalization".into(), "lf".into()),
            ("source_kind".into(), "text".into()),
        ],
        None => vec![
            ("byte_length".into(), byte_length),
            ("media_type".into(), media_type.clone()),
            ("source_kind".into(), "image".into()),
        ],
    });

    AlphaArtifact {
        declaration,
        execution,
        record: ArtifactRecord {
            id: ArtifactId(format!("artifact-{}", short_id(&hash.digest_hex))),
            hash,
            source: ValueRef(source.into()),
            metadata,
        },
        media_type,
        normalized_bytes,
        normalized_text,
    }
}

fn normalize_media_type(media_type: &str) -> String {
    media_type.trim().to_ascii_lowercase()
}

fn normalize_text(text: &str) -> String {
    let stripped = text.strip_prefix('\u{feff}').unwrap_or(text);
    stripped.replace("\r\n", "\n").replace('\r', "\n")
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push_str(&format!("{byte:02x}"));
    }
    output
}
