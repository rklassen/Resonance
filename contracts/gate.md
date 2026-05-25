# Gate Contract

Status: active contract  
Scope: Resonance observation readiness, phase-local verification, and stop/surface decisions

## Purpose

A `Gate` is an executable readiness condition.

A gate decides whether a component, payload, trace, process, or phase may be inspected, promoted, consumed downstream, or reported.

A gate is not a requirement, artifact, probe, payload, trace, claim, coding instruction, or final output. A requirement may demand a gate. A gate evaluates whether that requirement is satisfied in a specific context.

## Core Rule

A gate is defined by:

```text
gate id
subject contract
prerequisite gates
fitness function
phase scope
truth axes when applicable
pass/fail/blocked result
follow-up observation when below Yes
```

A gate must not contain hidden judgment logic or phase-inappropriate criteria.

## Gate vs Requirement

Requirement:

```text
what must be true
```

Gate:

```text
the executable check for whether it is currently true enough to proceed
```

Example:

```text
Requirement: same artifact must produce same hash.
Gate: run hash twice under the same normalization policy and compare.
```

## Required Gate Declaration

```rust
pub struct GateDeclaration {
    pub gate_id: GateId,
    pub display_name: String,

    pub subject_contract: ContractId,
    pub prerequisite_gate_ids: Vec<GateId>,
    pub fitness_function_id: FitnessFunctionId,

    pub phase_scope: Option<PhaseToken>,
    pub applies_to_requirement_ids: Vec<RequirementId>,
    pub truth_axes: Vec<TruthAxisId>,

    pub failure_policy: FailurePolicyId,
}
```

## Required Gate Result

```rust
pub struct GateResult {
    pub gate_result_id: GateResultId,
    pub gate_id: GateId,

    pub subject_ref: SubjectRef,
    pub prerequisite_results: Vec<GateResultId>,

    pub axis_results: Vec<TruthAxisResult>,
    pub decision: GateDecision,

    pub follow_up_observation: Option<ObservationRequest>,
    pub evidence_payload_ids: Vec<PayloadId>,
    pub evidence_trace_ids: Vec<TraceId>,

    pub created_at_utc: String,
}
```

## Decision States

```rust
pub enum GateDecision {
    Pass,
    Fail,
    Blocked,
    Deferred,
}
```

Rules:

1. `Pass` allows the declared downstream action.
2. `Fail` means the subject was evaluated and did not satisfy the gate.
3. `Blocked` means the gate could not run because a prerequisite, source, contract, or dependency is missing.
4. `Deferred` means the gate is not valid for the current phase.

## Truth Axes

Baseline truth axes:

```text
Integration
Performance
Accuracy
```

Rules:

1. Integration evaluates completeness, determinism, transparency, and declared graph connectivity.
2. Performance evaluates runtime/resource fitness against declared runtime policy.
3. Accuracy evaluates phase-valid correctness, not final scientific truth.
4. The composite truth scalar uses geometric mean, not arithmetic mean.
5. A zero on any governing axis is a stop-and-surface event.
6. No axis below `Yes` may be reported without an explicit named follow-up observation.

## Axis Scale

Use the smallest sufficient scale:

```text
No
Partial
Yes
NotApplicable
Blocked
```

Do not invent high-resolution confidence unless the evidence supports it.

## Geometric Mean Rule

For governing numeric axes:

```text
composite = geometric_mean(integration, performance, accuracy)
```

A zero on any governing axis makes the composite zero.

This prevents a high score on one axis from hiding complete failure on another.

## Phase Discipline

Gates are phase-local.

Rules:

1. Alpha gates verify shape and deterministic totality.
2. Beta gates verify privileged-path correctness.
3. Gamma gates verify discovery-apparatus behavior.
4. A gate must not judge alpha against gamma-only criteria.
5. Shape verification must not be reported as truth verification.
6. If a gate is invalid for the current phase, return `Deferred`, not `Fail`.

## Prerequisite Discipline

A downstream gate must not run until prerequisite gates pass or explicitly produce acceptable phase-local deferrals.

Example order:

```text
ArtifactHashGate
→ ProbeReplayGate
→ PayloadContractGate
→ ProjectionRangeGate
→ PriorAlignmentGate
→ GraphDimensionGate
→ RuntimeReplayGate
→ TraceSupportGate
→ ReportClaimGate
```

## Stop-and-Surface Rule

A gate must stop and surface when:

```text
governing axis is No
required prerequisite is missing
source provenance is missing
contract mismatch occurs
NaN or Inf appears in numeric runtime
hidden input is detected
mock is being used as truth
phase-inappropriate inspection is attempted
```

Stop-and-surface means no silent fallback, no inferred pass, and no hidden downgrade.

## Follow-Up Observation

If any axis is below `Yes`, the gate result must include a named follow-up observation that could raise it.

Example:

```rust
pub struct ObservationRequest {
    pub observation_id: ObservationId,
    pub description: String,
    pub required_subject: SubjectRef,
    pub expected_resolution: String,
}
```

## Valid Transitions

Allowed:

```text
GateDeclaration -> GateResult
GateResult -> Trace
GateResult -> PhasePromotion
GateResult -> ReportAssembly
```

Disallowed:

```text
GateResult -> hidden runtime override
GateResult -> direct final truth
GateResult -> silently ignored failure
GateResult -> phase escalation without promotion edge
```

## Snap Encoding

Gate declarations may be object nodes when they need graph identity:

```snap
object {
 id: g201,
 name: 'Gate-Artifact-Hash-Stability',
 type: GateDeclaration,
}
```

Gate evaluators are operator nodes:

```snap
operator {
 id: v201,
 in: { target: Artifact, },
 name: 'verify-artifact-hash-stability',
 out: { report: GateResult, },
}
```

Gate results use `❇`:

```text
Gate❇pass
Gate❇fail
Gate❇blocked
Gate❇deferred
```

Phase suffixes apply only to the gate declaration or result lifecycle:

```text
Gate-Α.shape
Gate-Β.privileged-path
Gate-Γ.discovery
```

## Required Baseline Gates

### G1 — Artifact Identity Gate

Checks stable hash and deterministic metadata.

### G2 — Probe Replay Gate

Checks replay within declared tolerance.

### G3 — Payload Contract Gate

Checks payload contract, producer, source, value ref, and numeric policy.

### G4 — Intermediate State Gate

Checks range, signed-vector validity, role constraints, and blocked invalid states.

### G5 — Prior Bridge Gate

Checks source map provenance, transform provenance, and parcel alignment.

### G6 — Graph Runtime Gate

Checks dimensions, non-dangling ids, no NaN, no Inf, and replayable transition.

### G7 — Trace Support Gate

Checks that every visible claim is linked to source records, payloads, Snap refs, or gate results.

### G8 — Phase Promotion Gate

Checks that mutations are explicit: promoted, wrapped, replaced, extended, stress-tested, transformed, removed, or deferred.

### G9 — Phase-Local Report Gate

Checks that reports evaluate only requirements valid for the current phase.

## Failure Modes

Block gate acceptance when:

1. gate has no subject contract,
2. gate has no fitness function,
3. prerequisite gate is missing,
4. phase scope is ambiguous,
5. failure policy is missing,
6. below-Yes result lacks follow-up observation,
7. alpha is judged against gamma-only criteria,
8. shape verification is treated as truth verification,
9. zero axis is softened into a nonzero composite,
10. gate result is ignored by downstream execution.

## Rust Sketch

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GateId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GateResultId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FitnessFunctionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RequirementId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TruthAxisId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FailurePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubjectRef(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ObservationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GateDecision {
    Pass,
    Fail,
    Blocked,
    Deferred,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AxisJudgment {
    No,
    Partial,
    Yes,
    NotApplicable,
    Blocked,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TruthAxisResult {
    pub axis_id: TruthAxisId,
    pub judgment: AxisJudgment,
    pub numeric_value: Option<f32>,
    pub evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservationRequest {
    pub observation_id: ObservationId,
    pub description: String,
    pub required_subject: SubjectRef,
    pub expected_resolution: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateDeclaration {
    pub gate_id: GateId,
    pub display_name: String,
    pub subject_contract: ContractId,
    pub prerequisite_gate_ids: Vec<GateId>,
    pub fitness_function_id: FitnessFunctionId,
    pub phase_scope: Option<PhaseToken>,
    pub applies_to_requirement_ids: Vec<RequirementId>,
    pub truth_axes: Vec<TruthAxisId>,
    pub failure_policy: FailurePolicyId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GateResult {
    pub gate_result_id: GateResultId,
    pub gate_id: GateId,
    pub subject_ref: SubjectRef,
    pub prerequisite_results: Vec<GateResultId>,
    pub axis_results: Vec<TruthAxisResult>,
    pub decision: GateDecision,
    pub follow_up_observation: Option<ObservationRequest>,
    pub evidence_payload_ids: Vec<PayloadId>,
    pub evidence_trace_ids: Vec<TraceId>,
    pub created_at_utc: String,
}
```

## Fitness Functions

```rust
pub fn fitness_gate_declaration_complete(
    gate: &GateDeclaration
) -> FitnessResult;

pub fn fitness_gate_prerequisites_satisfied(
    gate: &GateDeclaration,
    prior_results: &[GateResult]
) -> FitnessResult;

pub fn fitness_gate_phase_local(
    gate: &GateDeclaration,
    active_phase: &PhaseToken
) -> FitnessResult;

pub fn fitness_gate_truth_axes_valid(
    result: &GateResult
) -> FitnessResult;

pub fn fitness_gate_follow_up_present_when_needed(
    result: &GateResult
) -> FitnessResult;

pub fn fitness_gate_zero_axis_stops(
    result: &GateResult
) -> FitnessResult;
```

## Acceptance

The gate contract is accepted when gates are executable readiness checks that preserve phase discipline, block premature inspection, expose failures, require follow-up observations for below-Yes axes, and prevent temporary shape verification from being mistaken for final truth.
