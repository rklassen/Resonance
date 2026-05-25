# Claim Contract

Status: active contract  
Scope: Resonance reportable statements, blocked statements, and uncertainty-bearing conclusions

## Purpose

A `Claim` is a reportable statement whose support, status, and uncertainty are explicit.

A claim is not an artifact, payload, probe, operator, gate, trace, requirement, temporary state, or final output. Claims may appear inside traces and outputs only when supported, blocked, or deferred explicitly.

## Core Rule

A claim is defined by:

```text
claim id
statement
claim status
support records
uncertainty record
phase scope
blocked/deferred reason when applicable
```

It is not defined by a `kind` enum.

## Claim Status

```rust
pub enum ClaimStatus {
    ObservedFact,
    DerivedClaim,
    BlockedClaim,
    DeferredClaim,
}
```

This enum is report status, not semantic kind.

Rules:

1. `ObservedFact` must be directly grounded in artifact metadata, payload value, gate result, Snap structure, or runtime record.
2. `DerivedClaim` must name the operator path that produced it.
3. `BlockedClaim` must name the blocker.
4. `DeferredClaim` must name the missing phase, gate, prior, transform, or observation.

## Required Record

```rust
pub struct ClaimRecord {
    pub claim_id: ClaimRecordId,
    pub statement: String,

    pub status: ClaimStatus,
    pub phase_scope: Option<PhaseToken>,

    pub support_artifact_ids: Vec<ArtifactId>,
    pub support_payload_ids: Vec<PayloadId>,
    pub support_trace_ids: Vec<TraceId>,
    pub support_gate_result_ids: Vec<GateResultId>,
    pub support_snap_refs: Vec<SnapRef>,

    pub uncertainty: Option<UncertaintyRecord>,
    pub blocker: Option<BlockerRecord>,

    pub created_at_utc: String,
}
```

## Uncertainty

Claims must avoid false certainty.

Use uncertainty fields when evidence is incomplete, indirect, conflicting, or inferential:

```rust
pub struct UncertaintyRecord {
    pub belief: Option<f32>,
    pub plausibility: Option<f32>,
    pub confidence: Option<f32>,
    pub conflict: Option<f32>,
    pub unsupported_mass: Option<f32>,
}
```

Rules:

1. Do not collapse uncertainty into one score unless justified.
2. Disagreement must remain observable.
3. Unsupported mass must be preserved when evidence does not support closure.
4. High-confidence claims require phase-valid support.

## Blockers

```rust
pub struct BlockerRecord {
    pub blocker_id: String,
    pub description: String,
    pub missing_dependency: Option<String>,
    pub failed_gate_id: Option<GateId>,
    pub prohibited_edge: Option<String>,
    pub required_follow_up: Option<String>,
}
```

Blocked claims are useful. They prevent unsupported inference from becoming fake truth.

## Valid Transitions

Allowed:

```text
Payload -> Operator -> ClaimRecord
GateResult -> ClaimRecord
Trace -> ClaimRecord
ClaimRecord -> Output
```

Disallowed:

```text
ProbeExecution -> direct final claim
MockPayload -> truth claim
PriorField -> clinical measurement claim
Claim -> hidden runtime behavior
```

## Prohibited Claims

The system must not claim:

```text
complete causal model of humanstate
clinical diagnosis
treatment implication
actual NT concentration measurement
fabricated label or prior
final truth from probe output alone
truth from alpha mock path
```

## Phase Discipline

Claims are phase-local.

Rules:

1. Alpha claims may establish deterministic shape and traceability.
2. Alpha claims may not establish final causal truth.
3. Beta claims may establish privileged-path correctness.
4. Gamma claims may establish discovery-apparatus behavior.
5. A future-phase claim in an earlier phase must be deferred or blocked.

## Snap Encoding

Claims may be object nodes when graph identity matters:

```snap
object {
 id: c201,
 name: 'Claim-Β.replayable-parcel-runtime',
 type: ClaimRecord,
}
```

Claim views use `❇`:

```text
Claim❇support
Claim❇blocker
Claim❇uncertainty
```

Phase suffixes scope the claim record:

```text
Claim-Α.shape
Claim-Β.privileged-path
Claim-Γ.discovery
```

## Verification Gates

A claim is valid only if:

1. it has a status,
2. it has support records or a blocker,
3. derived claims name operator path,
4. observed facts are directly grounded,
5. blocked claims name blocker,
6. deferred claims name future gate/phase/dependency,
7. uncertainty is present when evidence is inferential,
8. phase scope is clear,
9. prohibited claims are blocked,
10. claim appears in output only through traceable record.

## Failure Modes

Block claim acceptance when:

1. statement has no support,
2. claim status is missing,
3. derived claim lacks operator path,
4. probe output is treated as final truth,
5. mock path is treated as truth,
6. prior field is called a measurement,
7. clinical implication appears,
8. uncertainty is hidden,
9. disagreement is erased,
10. future-phase requirement is applied prematurely.

## Rust Sketch

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClaimRecordId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClaimStatus {
    ObservedFact,
    DerivedClaim,
    BlockedClaim,
    DeferredClaim,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncertaintyRecord {
    pub belief: Option<f32>,
    pub plausibility: Option<f32>,
    pub confidence: Option<f32>,
    pub conflict: Option<f32>,
    pub unsupported_mass: Option<f32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockerRecord {
    pub blocker_id: String,
    pub description: String,
    pub missing_dependency: Option<String>,
    pub failed_gate_id: Option<GateId>,
    pub prohibited_edge: Option<String>,
    pub required_follow_up: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClaimRecord {
    pub claim_id: ClaimRecordId,
    pub statement: String,
    pub status: ClaimStatus,
    pub phase_scope: Option<PhaseToken>,
    pub support_artifact_ids: Vec<ArtifactId>,
    pub support_payload_ids: Vec<PayloadId>,
    pub support_trace_ids: Vec<TraceId>,
    pub support_gate_result_ids: Vec<GateResultId>,
    pub support_snap_refs: Vec<SnapRef>,
    pub uncertainty: Option<UncertaintyRecord>,
    pub blocker: Option<BlockerRecord>,
    pub created_at_utc: String,
}
```

## Acceptance

The claim contract is accepted when reportable statements are statused, supported, phase-local, uncertainty-aware, and unable to smuggle unsupported interpretation, clinical meaning, or future-phase truth into current outputs.
