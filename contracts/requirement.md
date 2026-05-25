# Requirement Contract

Status: active contract  
Scope: Resonance requirements, acceptance conditions, and invariant obligations

## Purpose

A `Requirement` is a declared obligation the system must satisfy.

A requirement states what must be true. It is not the executable check itself, not an artifact, not a payload, not a probe, not a process, not a trace, not a claim, and not a final output.

A requirement may be phase-local or invariant across all phases.

## Core Rule

A requirement is defined by:

```text
requirement id
statement
scope
subject contracts
verification gates
phase applicability
failure consequence
status
```

It is not defined by a `kind` enum.

## Requirement vs Gate

Requirement:

```text
what must be true
```

Gate:

```text
executable readiness check for whether it is true in a specific context
```

A requirement without a gate is incomplete unless explicitly marked as a non-executable workshop note.

## Required Record

```rust
pub struct RequirementRecord {
    pub requirement_id: RequirementId,
    pub display_name: String,
    pub statement: String,

    pub subject_contracts: Vec<ContractId>,
    pub applies_to_phases: Vec<PhaseToken>,
    pub verification_gate_ids: Vec<GateId>,

    pub parent_requirement_ids: Vec<RequirementId>,
    pub dependency_requirement_ids: Vec<RequirementId>,

    pub failure_policy: FailurePolicyId,
    pub status: RequirementStatus,
    pub created_at_utc: String,
}
```

## Requirement Status

```rust
pub enum RequirementStatus {
    Active,
    Deferred,
    Removed,
    Superseded,
}
```

Rules:

1. `Active` requirements are binding.
2. `Deferred` requirements are not binding in the current phase.
3. `Removed` requirements must preserve the removal reason.
4. `Superseded` requirements must reference the replacement requirement.

This enum is lifecycle state, not semantic kind.

## Phase Applicability

Phase tokens are cognitive handles, not a fixed lifecycle.

Rules:

1. A requirement may apply to one phase, many phases, or all phases.
2. Alpha must not be judged against gamma-only requirements.
3. Shape verification must not be treated as truth verification.
4. A requirement that applies to all phases should explicitly state that scope.

## Subject Contracts

Each requirement must name the contracts it governs.

Examples:

```text
contract.artifact
contract.probe
contract.payload
contract.operator
contract.state
contract.trace
contract.gate
contract.output
contract.claim
```

Rules:

1. Requirements must not float unattached to implementation subjects.
2. Requirements that govern multiple subjects must name each subject.
3. Ambiguous subject scope blocks implementation.

## Verification Mapping

Every executable requirement must map to at least one gate.

Example:

```text
Requirement: Probe outputs must remain separate.
Gate: verify-probe-output-separation.
```

Rules:

1. Missing gate blocks promotion to requirements-complete status.
2. A gate may verify multiple requirements.
3. A requirement may require multiple gates.
4. Gate results must be phase-local.

## Failure Policy

A requirement must declare what happens when it fails.

Baseline policies:

```text
stop-and-surface
block-downstream-use
defer-to-next-phase
warn-only
remove-as-violative
```

Use `warn-only` sparingly. Requirements that protect provenance, determinism, hidden inputs, fabricated data, phase discipline, or clinical boundaries should not be warn-only.

## Snap Encoding

Requirements may be encoded as object nodes:

```snap
object {
 id: q201,
 name: 'Requirement-Deterministic-Replay',
 type: Requirement,
}
```

Gate evaluators remain operators:

```snap
operator {
 id: v201,
 in: { target: TraceRecord, },
 name: 'verify-deterministic-replay',
 out: { report: GateResult, },
}
```

Requirement payloads or views use `❇`:

```text
Requirement❇gate-map
Requirement❇status
Requirement❇failure-policy
```

Phase suffixes apply only to phase-local requirement records:

```text
Requirement-Α.shape
Requirement-Β.privileged-path
Requirement-Γ.discovery
```

## Verification Gates

A requirement record is valid only if:

1. it has an id,
2. it has a precise statement,
3. it names subject contracts,
4. it names applicable phases,
5. it maps to verification gates or is explicitly non-executable,
6. it declares failure policy,
7. it has lifecycle status,
8. superseded records reference replacements,
9. removed records preserve removal reason,
10. no future-phase requirement is applied prematurely.

## Failure Modes

Block requirement acceptance when:

1. requirement has no subject,
2. requirement has no gate,
3. phase applicability is ambiguous,
4. failure policy is missing,
5. status is unclear,
6. statement mixes requirement with implementation instruction,
7. statement mixes requirement with final claim,
8. alpha is evaluated against gamma-only requirement,
9. requirement is silently overwritten,
10. removed or superseded requirement loses provenance.

## Rust Sketch

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequirementId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ContractId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhaseToken(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GateId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FailurePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequirementStatus {
    Active,
    Deferred,
    Removed,
    Superseded,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequirementRecord {
    pub requirement_id: RequirementId,
    pub display_name: String,
    pub statement: String,
    pub subject_contracts: Vec<ContractId>,
    pub applies_to_phases: Vec<PhaseToken>,
    pub verification_gate_ids: Vec<GateId>,
    pub parent_requirement_ids: Vec<RequirementId>,
    pub dependency_requirement_ids: Vec<RequirementId>,
    pub failure_policy: FailurePolicyId,
    pub status: RequirementStatus,
    pub created_at_utc: String,
}
```

## Acceptance

The requirement contract is accepted when requirements are precise, phase-aware, gate-linked obligations that remain separate from processes, payloads, claims, traces, coding instructions, and final outputs.
