# Probe Contract

Status: active contract  
Scope: Resonance evaluator and frozen-model inspection processes

## Purpose

A `Probe` is an executable evaluator process.

It consumes declared inputs, applies a frozen model, deterministic transform, zero-shot label set, or explicit evaluator, and emits one or more immutable payloads.

A probe is not a coding instruction, artifact, payload, requirement, gate, claim, final output, or clinical instrument.

## Core Rule

A probe is defined by:

```text
input contracts
output contracts
declared capabilities
model / prompt / preprocessing provenance
runtime policy
tolerance policy
overfit policy
execution record
```

It is **not** defined by a `kind` enum.

## Hard Separation

### Probe vs coding apparatus

Coding-agent instructions belong in:

```text
procedure/
plan/
implementation notes
developer prompts
```

Probe declarations belong in:

```text
contracts/
registry/
Snap operators
runtime manifests
```

A coding instruction may tell an agent to implement a probe wrapper. The instruction is not the probe.

### Probe vs payload

A probe is the process.  
A payload is what the process emits.

```text
ProbeExecution -> PayloadRecord
```

Never encode emitted values into the probe identity.

## Required Declaration

```rust
pub struct ProbeDeclaration {
    pub probe_id: ProbeId,
    pub display_name: String,

    pub input_contracts: Vec<ContractId>,
    pub output_contracts: Vec<ContractId>,
    pub declared_capabilities: Vec<CapabilityId>,

    pub model_ref: Option<ModelRef>,
    pub prompt_policy: Option<PolicyId>,
    pub preprocessing_policy: PolicyId,
    pub runtime_policy: RuntimePolicyId,
    pub tolerance_policy: TolerancePolicyId,
    pub overfit_policy: OverfitPolicyId,

    pub phase_scope: Option<PhaseToken>,
}
```

## Required Execution Record

```rust
pub struct ProbeExecutionRecord {
    pub execution_id: ExecutionId,
    pub probe_id: ProbeId,

    pub input_artifact_id: Option<ArtifactId>,
    pub input_payload_ids: Vec<PayloadId>,

    pub model_hash: Option<HashDigest>,
    pub prompt_hash: Option<HashDigest>,
    pub preprocessing_hash: HashDigest,
    pub runtime_policy: RuntimePolicyId,
    pub tolerance_policy: TolerancePolicyId,

    pub output_payload_ids: Vec<PayloadId>,
    pub created_at_utc: String,
}
```

## Capability Semantics

Capabilities are declared abilities bound to input and output contracts. They are not classifier buckets.

Examples:

```text
capability.visual-embedding
capability.zero-shot-label-response
capability.affective-axis-response
capability.color-field-summary
capability.material-scene-response
capability.compatibility-response
capability.deterministic-contract-check
```

A capability declaration must include:

```rust
pub struct ProbeCapability {
    pub capability_id: CapabilityId,
    pub description: String,
    pub required_input_contracts: Vec<ContractId>,
    pub declared_output_contracts: Vec<ContractId>,
    pub limitations: Vec<String>,
}
```

## Provenance Requirements

Every probe declaration must record:

```text
probe_id
input_contracts
output_contracts
declared_capabilities
preprocessing_policy
runtime_policy
tolerance_policy
overfit_policy
```

Every model-backed probe must also record:

```text
model_id
model_hash
model_source
model_license_ref when available
```

Every prompt-backed probe must also record:

```text
prompt_policy
prompt_hash
label_set_hash when applicable
```

Hidden model state, hidden preprocessing, hidden prompt changes, hidden runtime options, and hidden adaptation are prohibited.

## Execution Semantics

Allowed:

```text
Artifact -> ProbeExecution -> Payload
Payload -> ProbeExecution -> Payload
```

Disallowed:

```text
ProbeExecution -> final truth
ProbeExecution -> clinical claim
ProbeExecution -> direct receptor bridge
ProbeExecution -> direct graph runtime
ProbeExecution -> opaque aggregate score
```

Correct route:

```text
Artifact
→ ProbeExecution
→ ProbePayload
→ IntermediateStateProjection
→ PriorBridge
→ GraphRuntime
→ Trace
→ Output
```

## Overfit and Leakage Controls

A probe is not automatically trustworthy because it produces a clean result.

The system must track:

```text
representation-inspection risk
prompt sensitivity
label leakage
memorization
model-family agreement illusion
domain mismatch
capacity overfit
aggregation overfit
```

### Frozen probes

Frozen probes are baseline.

Required controls:

```text
model provenance
prompt provenance
input preprocessing provenance
repeatability check
cross-model disagreement check when available
prompt-variant sensitivity check when available
```

### Learned probes

Learned probes are not baseline.

If a learned probe is introduced, it must include:

```text
training data provenance
held-out evaluation
negative control or control task
capacity constraint
selectivity report
```

A learned probe must not be promoted merely because it scores well. Probe success can reflect probe capacity rather than information already present in the inspected representation.

## Aggregation

Probe outputs remain separate unless an explicit aggregation operator creates a new payload.

Aggregation must preserve:

```text
source payload ids
aggregation operator id
weights or rules
confidence
conflict
unsupported mass
```

No hidden merge into a single opaque score is allowed.

## Snap Encoding

Executable probes should be encoded as `operator` nodes.

```snap
operator {
 id: p201,
 in: { artifact: Artifact, },
 name: 'Visual-Embedding-Probe-Α',
 out: { payload: Payload, },
}
```

Probe manifests may be encoded as `object` nodes.

```snap
object {
 id: m201,
 name: 'Probe-Manifest-Β',
 type: ProbeManifest,
}
```

Payload names use `❇`:

```text
Visual-Embedding-Probe❇vector
Affective-Axis-Probe❇logits
```

Phase suffixes describe lifecycle state of the probe declaration or manifest:

```text
Visual-Embedding-Probe-Α.mock
Visual-Embedding-Probe-Β.verified
Probe-Suite-Γ.expanded
```

## Verification Gates

A probe is valid only if:

1. declaration includes probe id, input contracts, output contracts, capabilities, preprocessing policy, runtime policy, tolerance policy, and overfit policy,
2. all model-backed probes declare model provenance,
3. all prompt-backed probes declare prompt provenance,
4. all material dependencies are included in replay/cache identity,
5. repeated execution is stable within declared tolerance,
6. no hidden adaptation occurs,
7. every output is an immutable payload,
8. outputs remain separate unless explicitly aggregated,
9. overfit controls are declared before high-confidence use,
10. probe results do not directly emit final truth claims.

## Failure Modes

Block execution or promotion when:

1. input contract is missing,
2. output contract is missing,
3. capability is undeclared,
4. model hash is missing for model-backed probe,
5. prompt hash is missing for prompt-backed probe,
6. preprocessing is undeclared,
7. runtime policy is undeclared,
8. hidden adaptation is detected,
9. learned probe lacks overfit controls,
10. prompt sensitivity is unmeasured but confidence is high,
11. output collapses into an opaque score,
12. probe result is treated as final truth.

## Rust Sketch

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProbeId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContractId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TolerancePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OverfitPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhaseToken(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HashDigest {
    pub algorithm: String,
    pub digest_hex: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelRef {
    pub model_id: String,
    pub model_hash: HashDigest,
    pub model_source: Option<String>,
    pub model_license_ref: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeCapability {
    pub capability_id: CapabilityId,
    pub description: String,
    pub required_input_contracts: Vec<ContractId>,
    pub declared_output_contracts: Vec<ContractId>,
    pub limitations: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeDeclaration {
    pub probe_id: ProbeId,
    pub display_name: String,
    pub input_contracts: Vec<ContractId>,
    pub output_contracts: Vec<ContractId>,
    pub declared_capabilities: Vec<CapabilityId>,
    pub model_ref: Option<ModelRef>,
    pub prompt_policy: Option<PolicyId>,
    pub preprocessing_policy: PolicyId,
    pub runtime_policy: RuntimePolicyId,
    pub tolerance_policy: TolerancePolicyId,
    pub overfit_policy: OverfitPolicyId,
    pub phase_scope: Option<PhaseToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExecutionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArtifactId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PayloadId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProbeExecutionRecord {
    pub execution_id: ExecutionId,
    pub probe_id: ProbeId,
    pub input_artifact_id: Option<ArtifactId>,
    pub input_payload_ids: Vec<PayloadId>,
    pub model_hash: Option<HashDigest>,
    pub prompt_hash: Option<HashDigest>,
    pub preprocessing_hash: HashDigest,
    pub runtime_policy: RuntimePolicyId,
    pub tolerance_policy: TolerancePolicyId,
    pub output_payload_ids: Vec<PayloadId>,
    pub created_at_utc: String,
}
```

## Fitness Functions

```rust
pub fn fitness_probe_declaration_complete(
    probe: &ProbeDeclaration
) -> FitnessResult;

pub fn fitness_probe_provenance_complete(
    execution: &ProbeExecutionRecord
) -> FitnessResult;

pub fn fitness_probe_replay_within_tolerance(
    prior: &ProbeExecutionRecord,
    replay: &ProbeExecutionRecord
) -> FitnessResult;

pub fn fitness_probe_overfit_policy_declared(
    probe: &ProbeDeclaration
) -> FitnessResult;

pub fn fitness_probe_no_hidden_adaptation(
    probe: &ProbeDeclaration,
    execution: &ProbeExecutionRecord
) -> FitnessResult;

pub fn fitness_probe_outputs_are_payloads(
    execution: &ProbeExecutionRecord
) -> FitnessResult;
```

## Acceptance

The probe contract is accepted when:

1. probes are declared as evaluator processes, not kinds,
2. coding instructions are separated from evaluator declarations,
3. all material dependencies are recorded,
4. all outputs are immutable payloads,
5. probe outputs remain separate until explicitly aggregated,
6. overfit and sensitivity policies are declared,
7. replay is deterministic within tolerance,
8. no probe emits a final truth claim directly.
```
