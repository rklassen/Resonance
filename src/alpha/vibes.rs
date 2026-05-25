use crate::{
    CapabilityId, ContractId, ExecutionId, HashDigest, NumericPolicyId, OperatorDeclaration,
    OperatorExecutionRecord, OperatorId, PayloadId, PayloadRecord, PhaseToken, ProvenancePolicyId,
    RuntimePolicyId, SideEffectPolicyId, UtcMinute, ValueRef,
};

use super::artifact::{sha256_hex, short_id};
use super::probe::AlphaProbeRun;
use super::AlphaError;

const ALPHA_PHASE: &str = "Α";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AxisSpec {
    pub name: &'static str,
    pub role: &'static str,
    pub collapse_group: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectionValidation {
    pub ranges_valid: bool,
    pub collapse_valid: bool,
    pub roles_valid: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AlphaVibes {
    pub declaration: OperatorDeclaration,
    pub execution: OperatorExecutionRecord,
    pub payload_12d: PayloadRecord,
    pub payload_11d: PayloadRecord,
    pub signed_12d: [f32; 12],
    pub signed_11d: [f32; 11],
    pub validation: ProjectionValidation,
}

pub const VIBES_12D: [AxisSpec; 12] = [
    AxisSpec {
        name: "valence",
        role: "affect",
        collapse_group: "valence",
    },
    AxisSpec {
        name: "arousal",
        role: "affect",
        collapse_group: "arousal",
    },
    AxisSpec {
        name: "dominance",
        role: "affect",
        collapse_group: "dominance",
    },
    AxisSpec {
        name: "warmth",
        role: "scene",
        collapse_group: "warmth",
    },
    AxisSpec {
        name: "tension",
        role: "scene",
        collapse_group: "tension",
    },
    AxisSpec {
        name: "coherence",
        role: "stance",
        collapse_group: "coherence",
    },
    AxisSpec {
        name: "novelty",
        role: "stance",
        collapse_group: "novelty",
    },
    AxisSpec {
        name: "weight",
        role: "texture",
        collapse_group: "weight",
    },
    AxisSpec {
        name: "brightness",
        role: "texture",
        collapse_group: "surface",
    },
    AxisSpec {
        name: "roughness",
        role: "texture",
        collapse_group: "surface",
    },
    AxisSpec {
        name: "depth",
        role: "energy",
        collapse_group: "depth",
    },
    AxisSpec {
        name: "motion",
        role: "energy",
        collapse_group: "motion",
    },
];

pub fn project_to_vibes(
    embedding: &AlphaProbeRun,
    labels: &AlphaProbeRun,
) -> Result<AlphaVibes, AlphaError> {
    let signed_12d = std::array::from_fn(|index| {
        let embedding_value = embedding.values[index % embedding.values.len()];
        let label_value = labels.values[index % labels.values.len()];
        ((embedding_value * 0.6) + (label_value * 0.4)).clamp(-1.0, 1.0)
    });
    let signed_11d = collapse_to_11d(&signed_12d);
    let validation = validate_projection(&signed_12d, &signed_11d);
    if !(validation.ranges_valid && validation.collapse_valid && validation.roles_valid) {
        return Err(AlphaError::new("alpha vibes projection failed validation"));
    }

    let hash_12 = vector_hash(&signed_12d);
    let hash_11 = vector_hash(&signed_11d);
    let declaration = OperatorDeclaration {
        id: OperatorId("operator-alpha-vibes".into()),
        name: "project-alpha-vibes".into(),
        inputs: vec![embedding.payload.contract.clone(), labels.payload.contract.clone()],
        outputs: vec![
            ContractId("contract.payload.vibes-12d".into()),
            ContractId("contract.payload.vibes-11d".into()),
        ],
        capabilities: vec![CapabilityId("capability.vibes-projection".into())],
        runtime: RuntimePolicyId("runtime.alpha.deterministic".into()),
        determinism: crate::DeterminismPolicyId("determinism.replayable".into()),
        side_effects: SideEffectPolicyId("side-effect.write-trace".into()),
        failure: crate::FailurePolicyId("failure.stop-and-surface".into()),
        phase: Some(PhaseToken(ALPHA_PHASE.into())),
    };
    let execution = OperatorExecutionRecord {
        id: ExecutionId(format!("execution-{}", short_id(&hash_12.digest_hex))),
        operator: declaration.id.clone(),
        input_artifacts: Vec::new(),
        input_payloads: vec![embedding.payload.id.clone(), labels.payload.id.clone()],
        output_payloads: vec![
            PayloadId(format!("payload-vibes12-{}", short_id(&hash_12.digest_hex))),
            PayloadId(format!("payload-vibes11-{}", short_id(&hash_11.digest_hex))),
        ],
        output_gate_results: Vec::new(),
        output_traces: Vec::new(),
        runtime: declaration.runtime.clone(),
        created: UtcMinute(202605240010),
    };

    Ok(AlphaVibes {
        declaration,
        payload_12d: PayloadRecord {
            id: execution.output_payloads[0].clone(),
            contract: ContractId("contract.payload.vibes-12d".into()),
            producer: execution.id.clone(),
            source_artifacts: Vec::new(),
            source_payloads: vec![embedding.payload.id.clone(), labels.payload.id.clone()],
            value: ValueRef("inline://alpha/vibes/12d".into()),
            hash: Some(hash_12),
            numeric: Some(NumericPolicyId("numeric.signed-vector.12d".into())),
            provenance: ProvenancePolicyId("provenance.explicit".into()),
            phase: Some(PhaseToken(ALPHA_PHASE.into())),
            created: UtcMinute(202605240010),
        },
        payload_11d: PayloadRecord {
            id: execution.output_payloads[1].clone(),
            contract: ContractId("contract.payload.vibes-11d".into()),
            producer: execution.id.clone(),
            source_artifacts: Vec::new(),
            source_payloads: vec![embedding.payload.id.clone(), labels.payload.id.clone()],
            value: ValueRef("inline://alpha/vibes/11d".into()),
            hash: Some(hash_11),
            numeric: Some(NumericPolicyId("numeric.signed-vector.11d".into())),
            provenance: ProvenancePolicyId("provenance.explicit".into()),
            phase: Some(PhaseToken(ALPHA_PHASE.into())),
            created: UtcMinute(202605240010),
        },
        execution,
        signed_12d,
        signed_11d,
        validation,
    })
}

fn collapse_to_11d(raw: &[f32; 12]) -> [f32; 11] {
    [
        raw[0],
        raw[1],
        raw[2],
        raw[3],
        raw[4],
        raw[5],
        raw[6],
        raw[7],
        ((raw[8] + raw[9]) * 0.5).clamp(-1.0, 1.0),
        raw[10],
        raw[11],
    ]
}

fn validate_projection(raw: &[f32; 12], collapsed: &[f32; 11]) -> ProjectionValidation {
    let ranges_valid = raw
        .iter()
        .chain(collapsed.iter())
        .all(|value| value.is_finite() && (-1.0..=1.0).contains(value));
    let collapse_valid = ((raw[8] + raw[9]) * 0.5 - collapsed[8]).abs() < 1e-6;
    let roles_valid = VIBES_12D.iter().all(|axis| {
        matches!(axis.role, "affect" | "scene" | "stance" | "texture" | "energy")
            && !axis.collapse_group.is_empty()
    });
    ProjectionValidation {
        ranges_valid,
        collapse_valid,
        roles_valid,
    }
}

fn vector_hash<const N: usize>(values: &[f32; N]) -> HashDigest {
    HashDigest {
        algorithm: "sha256".into(),
        digest_hex: sha256_hex(&[values
            .iter()
            .map(|value| format!("{value:+.6}"))
            .collect::<Vec<_>>()
            .join(",")
            .as_bytes()]),
    }
}
