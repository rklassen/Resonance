# Trace Contract

Status: active contract  
Scope: Resonance run provenance, replay evidence, and report support

## Purpose

A `Trace` is an immutable provenance record for one declared execution path.

A trace records what happened, which declared inputs and operators were involved, which payloads were produced or consumed, which Snap edges were traversed, which gates passed or failed, and which claims are supported or blocked.

A trace is not an artifact, probe, payload, requirement, gate, coding instruction, temporary state, or final output. It may contribute to final output, but it is not itself the system-level workshop output.

## Core Rule

A trace is defined by:

```text
run identity
phase token when applicable
artifact identity
operator executions
payload references
Snap path
gate results
claim support records
blocked claims
replay metadata
```

A trace must not contain hidden state, undeclared dependencies, or ungrounded interpretation.

## Required Record

```rust
pub struct TraceRecord {
    pub trace_id: TraceId,
    pub run_id: RunId,
    pub phase_scope: Option<PhaseToken>,

    pub source_artifact_ids: Vec<ArtifactId>,
    pub operator_execution_ids: Vec<ExecutionId>,
    pub payload_ids: Vec<PayloadId>,
    pub snap_path: SnapPathRef,

    pub gate_result_ids: Vec<GateResultId>,
    pub claim_record_ids: Vec<ClaimRecordId>,
    pub blocked_claim_ids: Vec<BlockedClaimId>,

    pub replay_policy: ReplayPolicyId,
    pub created_at_utc: String,
}
```

## Required Step Record

```rust
pub struct TraceStep {
    pub step_id: TraceStepId,
    pub trace_id: TraceId,

    pub operator_id: OperatorId,
    pub input_payload_ids: Vec<PayloadId>,
    pub output_payload_ids: Vec<PayloadId>,

    pub snap_node_refs: Vec<SnapNodeRef>,
    pub snap_edge_refs: Vec<SnapEdgeRef>,

    pub gate_result_ids: Vec<GateResultId>,
    pub started_at_utc: Option<String>,
    pub finished_at_utc: Option<String>,
}
```

## Required Claim Support Record

```rust
pub struct ClaimRecord {
    pub claim_id: ClaimRecordId,
    pub trace_id: TraceId,
    pub claim_text: String,

    pub claim_status: ClaimStatus,
    pub support_payload_ids: Vec<PayloadId>,
    pub support_gate_result_ids: Vec<GateResultId>,
    pub support_snap_refs: Vec<SnapRef>,

    pub uncertainty: Option<UncertaintyRecord>,
}
```

## Claim Status

```rust
pub enum ClaimStatus {
    ObservedFact,
    DerivedClaim,
    BlockedClaim,
    DeferredClaim,
}
```

Rules:

1. `ObservedFact` must be directly supported by artifact metadata, declared model output, gate result, Snap structure, or runtime record.
2. `DerivedClaim` must reference the operator and payload path used to derive it.
3. `BlockedClaim` must name the missing dependency, failed gate, unsupported prior, or prohibited edge.
4. `DeferredClaim` must name the phase or gate that could make it inspectable later.

## Trace vs Payload

Payloads carry values.

Traces record how values moved.

Allowed:

```text
Payload -> TraceStep
TraceStep -> TraceRecord
TraceRecord -> Output
```

Disallowed:

```text
Trace -> direct runtime value
Trace -> hidden payload
Trace -> ungrounded claim
```

If a trace needs to preserve a runtime value, it must reference a payload by id.

## Trace vs Output

`output/` is the system-level workshop deliverable directory.

A trace may be written under `output/traces/`, but it remains a trace record. A report or export may consume one or more traces to become a final output.

## Snap Path

Each trace must record enough Snap references to reconstruct the declared path.

Baseline references:

```text
Snap graph id
Snap node ids
Snap edge family names
Snap edge refs or reified edge records
operator refs
dynamic weight refs when used
```

If an edge cannot be directly identified by Snap syntax, it must be represented by a reified edge record.

## Required Trace Contents

Every run trace must include:

```text
artifact identity
probe identities
probe execution ids
projected state payload id
prior bridge payload id when present
graph transition payload id when present
Snap path
gate results
phase-valid pass/fail result
```

## Prohibited Contents

A trace must not include:

```text
clinical conclusion
unverified final truth
hidden operator behavior
undocumented runtime state
fabricated label or prior
opaque aggregate score
payload value without payload record
```

## Temporary / Phase-Scoped Traces

Phase-scoped traces are allowed.

Rules:

1. Alpha traces may include mocks only when mock policy is explicit.
2. Shape verification must be labeled as shape verification.
3. Mock-supported traces must not be used as truth verification.
4. Beta and later traces must distinguish replaced, deferred, and real prior paths.
5. Gamma traces must show when gamma-specific dynamics reduce to the verified beta substrate.

## Aggregated Traces

Aggregating traces creates a new trace or report record.

Aggregation must preserve:

```text
input trace ids
aggregation operator id
included phases
included gates
conflicts
blocked claims
unsupported mass
```

No aggregation may erase disagreement or failed gates.

## Snap Encoding

Trace records may be represented as object nodes when graph identity matters:

```snap
object {
 id: t201,
 name: 'Trace-Report-Β.replayable',
 type: TraceRecord,
}
```

Trace fragments use `❇`:

```text
Trace❇snap-path
Trace❇gate-results
Trace❇blocked-claims
Trace❇payload-lineage
```

Phase suffixes describe trace lifecycle state:

```text
Trace-Α.shape-proof
Trace-Β.verified-path
Trace-Γ.discovery-run
```

## Verification Gates

A trace is valid only if:

1. it has trace id and run id,
2. it records source artifact ids,
3. it records operator execution ids,
4. it records payload ids rather than embedding hidden values,
5. it records Snap path references,
6. it records gate results,
7. every visible claim is classified as observed, derived, blocked, or deferred,
8. every derived claim links to support payloads and operators,
9. every blocked claim names the blocker,
10. phase-local pass/fail is explicit.

## Failure Modes

Block trace acceptance when:

1. trace omits source artifact identity,
2. trace references unknown payload ids,
3. trace embeds runtime values without payload records,
4. trace omits Snap path,
5. trace includes unclassified claims,
6. trace reports alpha shape verification as truth verification,
7. trace hides failed gates,
8. trace erases disagreement,
9. trace contains clinical or unsupported humanstate claims,
10. trace cannot be replayed within declared replay policy.

## Rust Sketch

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RunId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TraceStepId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GateResultId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClaimRecordId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockedClaimId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapPathRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapNodeRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapEdgeRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SnapRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OperatorId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReplayPolicyId(pub String);

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
pub struct TraceRecord {
    pub trace_id: TraceId,
    pub run_id: RunId,
    pub phase_scope: Option<PhaseToken>,
    pub source_artifact_ids: Vec<ArtifactId>,
    pub operator_execution_ids: Vec<ExecutionId>,
    pub payload_ids: Vec<PayloadId>,
    pub snap_path: SnapPathRef,
    pub gate_result_ids: Vec<GateResultId>,
    pub claim_record_ids: Vec<ClaimRecordId>,
    pub blocked_claim_ids: Vec<BlockedClaimId>,
    pub replay_policy: ReplayPolicyId,
    pub created_at_utc: String,
}
```

## Fitness Functions

```rust
pub fn fitness_trace_has_sources(trace: &TraceRecord) -> FitnessResult;

pub fn fitness_trace_payloads_exist(trace: &TraceRecord) -> FitnessResult;

pub fn fitness_trace_snap_path_present(trace: &TraceRecord) -> FitnessResult;

pub fn fitness_trace_gate_results_present(trace: &TraceRecord) -> FitnessResult;

pub fn fitness_trace_claims_classified(trace: &TraceRecord) -> FitnessResult;

pub fn fitness_trace_no_hidden_values(trace: &TraceRecord) -> FitnessResult;

pub fn fitness_trace_phase_local(trace: &TraceRecord) -> FitnessResult;
```

## Acceptance

The trace contract is accepted when traces are immutable, replayable, provenance-bearing records that distinguish observed facts, derived claims, blocked claims, and deferred claims while linking every visible claim back to artifacts, probes, payloads, Snap edges, operators, gates, and phase-local requirements.
