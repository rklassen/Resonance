# State Contract

Status: active contract  
Scope: Resonance lifecycle condition, phase-local status, and controlled mutation

## Purpose

A `State` is the lifecycle or phase-local condition of a declared subject.

State describes whether an artifact, payload, operator, requirement, gate,
trace, claim, or output is mock, verified, blocked, deferred, exploratory,
promoted, replaced, or otherwise conditioned for the current phase.

State is not identity. State is not a payload value. State is not a
requirement, gate, trace, claim, or final output.

## Core Rule

A state is defined by:

```text
state id
subject reference
state label
phase scope when temporary
entry condition
exit condition
allowed mutation verbs
evidence refs
created at timestamp
```

It is not defined by a broad `kind` enum.

## State vs Identity

Identity tells what a subject is.

State tells what condition that subject is in.

Examples:

```text
Payload id: stable identity
Payload-Α.mock: phase-local state
Operator-Β.verified: lifecycle condition
Requirement-Γ.deferred: applicability state
```

Phase tokens qualify temporary condition. They must not become durable identity.

## Required Record

```rust
pub struct StateRecord {
    pub state_id: StateId,
    pub subject_ref: SubjectRef,
    pub state_label: StateLabel,

    pub phase_scope: Option<PhaseToken>,
    pub entered_by: Option<OperatorId>,
    pub exited_by: Option<OperatorId>,

    pub allowed_mutation_verbs: Vec<MutationVerb>,
    pub evidence_payload_ids: Vec<PayloadId>,
    pub evidence_gate_result_ids: Vec<GateResultId>,
    pub evidence_trace_ids: Vec<TraceId>,

    pub created_at_utc: String,
}
```

## State Labels

Use narrow labels with explicit meaning.

Examples:

```text
mock
stub
fixture-backed
exploratory
shape-verified
verified
blocked
deferred
replaced
promoted
removed-as-violative
```

Do not collapse these into one broad semantic bucket.

## Mutation Verbs

Allowed verbs:

```text
promote
wrap
replace
extend
stress-test
transform
remove
defer
block
```

Rules:

1. Mutation verbs describe lifecycle change, not ontology.
2. Every mutation must name the subject and supporting evidence.
3. No implicit upgrade path is allowed.
4. `defer` is valid only when the current phase cannot legitimately evaluate
   the subject.
5. `replace` must preserve a pointer to the replaced subject or state.

## Phase Discipline

Phase tokens are cognitive handles.

Rules:

1. Phase count is variable.
2. Phase token is not durable identity.
3. Temporary states must be phase-scoped.
4. Phase-local conditions must not leak forward or backward without an explicit
   mutation record.
5. Alpha shape-proof state must not be reported as beta correctness or gamma
   discovery truth.

## Mock and Deferred States

Mock, stub, and exploratory states are allowed when explicitly declared.

They must include:

```text
phase scope
source explanation
blocked-from-final-claim behavior
required gate or dependency for exit
```

Deferred state means the subject is not yet inspectable in the current phase.
It is not a silent failure and not a hidden pass.

## Valid Transitions

Allowed:

```text
mock -> shape-verified
shape-verified -> verified
verified -> extended
verified -> replaced
blocked -> deferred
deferred -> verified
verified -> removed-as-violative
```

Disallowed:

```text
mock -> final truth
blocked -> pass without evidence
deferred -> verified without gate result
phase-scoped state -> durable identity
```

## Snap Encoding

States may be represented as object nodes or payload-backed records when state
itself must be graph-visible.

```snap
object {
 id: s201,
 name: 'Payload-Α.mock',
 type: State,
}
```

State payloads or views use `❇` when emitting state summaries:

```text
State❇transition-map
State❇phase-scope
State❇blockers
```

## Verification Gates

A state record is valid only if:

1. it names a subject,
2. it declares a state label,
3. temporary states declare phase scope,
4. supporting evidence is explicit,
5. exit conditions are declared for mock, blocked, or deferred states,
6. mutation verbs are allowed for that subject,
7. phase-local state is not reported as durable identity,
8. mocks are blocked from final truth claims.

## Failure Modes

Block state promotion when:

1. state label is ambiguous,
2. phase scope is missing for a temporary condition,
3. mock state is treated as verified truth,
4. blocked state is silently consumed downstream,
5. deferred state is reported as failure rather than deferral,
6. mutation occurs without evidence,
7. a phase token is used as the subject's identity.

## Acceptance

The state contract is accepted when temporary, blocked, deferred, mock, and
verified conditions are explicit, phase-scoped when necessary, and unable to
masquerade as identity, payload value, or final truth.