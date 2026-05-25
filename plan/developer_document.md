# Resonance Developer Handoff

Status: current working handoff  
Audience: coding agent / developer agent  
Purpose: one entry document that summarizes the current plan, missing gaps, and required implementation discipline

## 1. Current State

Resonance is a workshop study and implementation project.

The active product is a deterministic frozen-probe inspection machine:

```text
artifact
→ frozen probes
→ cached latent payloads
→ Vibes-style intermediate state
→ prior bridge
→ parcel / highway graph runtime
→ disagreement and width readout
→ Snap trace
→ phase-local report / output
```

The system is not:

```text
training system
clinical instrument
human-labeling loop
complete causal model of humanstate
direct NT concentration measurement
oracle of final truth
```

The central purpose is to reinforce a humanstate metric with visual sentiment / learned-representation probes represented as auxiliary chains in a typed directed knowledge graph.

## 2. Source Basis

The developer should treat the following as source documents:

```text
specification/requirements.md
contracts/readme.md
contracts/artifact.md
contracts/probe.md
contracts/payload.md
contracts/operator.md
contracts/state.md
contracts/requirement.md
contracts/gate.md
contracts/trace.md
contracts/claim.md
contracts/output.md
plan/implementation-plan.snap
plan/work-package-01-api.md
```

The existing `requirements.md` is a good base and should not be decimated. It should be updated by folding in the requirement ontology / contract alignment section, not rewritten from scratch.

## 3. Immediate Correction: Stale Implementation Plan

The current `plan/implementation-plan.snap` still shows an older fixed-phase framing:

```text
gamma-phase-evolution
Alpha / Beta / Gamma as apparent fixed lifecycle
```

That is stale.

Correct interpretation:

```text
phase tokens are cognitive handles
phase count is variable
Alpha/Beta/Gamma are examples only
trivial work may use one phase
complex work may use many phases
```

Rename or reinterpret the plan around Resonance:

```text
resonance-implementation-plan
```

Use:

```snap
operators: 'ops/resonance/',
workspace: './resonance/',
```

Keep UTC timestamps absolute:

```snap
time: YYYY-MM-DDTHH:MM:SSZ
```

## 4. Non-Negotiable Category Separation

Do not commingle these roles:

| Role | Meaning | Not |
|---|---|---|
| Artifact | source entity inspected by Resonance | payload, process, output, claim |
| Probe | evaluator process | coding instruction, emitted value, truth |
| Payload | immutable runtime currency | artifact, process, trace, output |
| Operator | executable process | payload, developer instruction |
| State | lifecycle / phase-local condition | identity, payload, requirement |
| Requirement | obligation: what must be true | executable check |
| Gate | executable readiness / verification check | requirement text |
| Trace | provenance record of what happened | hidden payload storage |
| Claim | reportable statement with support/blocker | truth by default |
| Output | system-level workshop deliverable | local `.out`, payload |

This separation must exist before runtime logic.

## 5. No Broad `Kind` Enum Rule

Do not introduce broad semantic bucket enums:

```text
ProbeKind
PayloadKind
ArtifactKind
TraceKind
RequirementKind
OutputKind
```

Use:

```text
ContractId
CapabilityId
PolicyId
StateLabel
ClaimStatus
GateDecision
RequirementStatus
```

Rationale: broad `Kind` enums hide semantics and create migration cost.

## 6. Coding Apparatus vs Evaluator Apparatus

Coding instructions are not probes.

Coding instructions belong in:

```text
plan/
procedure/
implementation notes
developer prompts
```

Evaluator declarations belong in:

```text
contracts/
registry/
Snap operators
runtime manifests
```

A prompt saying “implement a probe wrapper” is not itself a probe.

## 7. Contract Set Status

The following contracts now exist or should exist exactly as the category layer:

```text
artifact.md
probe.md
payload.md
operator.md
state.md
requirement.md
gate.md
trace.md
claim.md
output.md
```

No new top-level contract category is needed unless implementation discovers a truly independent role.

If a proposed file overlaps an existing role, extend the existing contract instead of adding another noun.

## 8. Repository Shape

Recommended current repo shape:

```text
resonance/
  readme.md

  specification/
    requirements.md
    snap-spec-0.8.md
    ontology.md            # optional if not fully folded into requirements
    phase-model.md         # optional if not fully folded into requirements

  contracts/
    readme.md
    artifact.md
    probe.md
    payload.md
    operator.md
    state.md
    requirement.md
    gate.md
    trace.md
    claim.md
    output.md

  plan/
    implementation-plan.md
    implementation-plan.snap

  procedure/
    verify-implementation.md
    gate-checklist.md

  src/
    lib.rs
    bin/

  tests/
    integration/

  specimens/
    alpha/
    beta/
    gamma/

  output/
    traces/
    reports/
    exports/
    reviews/
```

Use `output/` only for system-level workshop deliverables, not local operator `.out`.

## 9. Implementation Order

Implement records and validators before runtime behavior.

Recommended order:

```text
1. artifact
2. payload
3. operator
4. probe
5. state
6. requirement
7. gate
8. trace
9. claim
10. output
```

Reasoning:

```text
source identity first
runtime currency second
executable process third
evaluator as specialized process fourth
lifecycle state before phase requirements
gates before trace/report interpretation
claims and outputs last
```

## 10. Minimum Questions Before Coding Any Struct

For every Rust struct, Snap node, operator, or procedure, answer:

```text
What role is this?
Which contract governs it?
What inputs are declared?
What outputs are declared?
Is it durable, temporary, or phase-scoped?
What gate verifies it?
What trace records it?
Can it support a claim?
Can it appear in final output?
```

If any answer is missing, the module is not ready.

## 11. Phase Model

Phase tokens are not ontology.

Rules:

```text
phase count is variable
phase token is not identity
phase exists only to reduce inference load, integration risk, runtime uncertainty, component ambiguity, or verification ambiguity
temporary states must be phase-scoped
phase-local requirements must not leak forward or backward
```

Allowed mutation verbs:

```text
promote
wrap
replace
extend
stress-test
transform
remove
defer
```

No implicit upgrade path is allowed.

## 12. First Implementation Milestone

The first useful milestone is not final truth.

It is:

```text
one artifact
→ at least two frozen probes
→ cache
→ intermediate state projection
→ mock prior gain
→ toy graph runtime
→ Snap-linked deterministic trace/report
```

This proves totality shape only.

It must not be interpreted as final model correctness.

## 13. Beta / Privileged Path Milestone

The next milestone is functional privileged path correctness:

```text
alpha mocks replaced or explicitly deferred
real priors installed through provenance-bearing transforms
parcel runtime replayable
reports evaluated only against beta-valid criteria
```

Do not require gamma discovery-surface behavior at beta.

## 14. Ultimate / Discovery Milestone

The ultimate discovery apparatus should expose:

```text
agreement
disagreement
width
energy
traceable operator paths
blocked claims
deferred claims
uncertainty
```

No training, clinical claims, hidden inputs, manual bottlenecks, or unsupported priors.

## 15. Gate Discipline

Gates are executable readiness checks.

Gate result states:

```text
Pass
Fail
Blocked
Deferred
```

Truth axes:

```text
Integration
Performance
Accuracy
```

Rules:

```text
composite truth scalar uses geometric mean
zero on any governing axis is stop-and-surface
any axis below Yes requires a named follow-up observation
shape verification is not truth verification
alpha must not be judged against gamma-only criteria
```

## 16. Trace Discipline

Every visible output must be trace-supported.

A trace must link:

```text
artifact identity
operator executions
payload ids
Snap path
gate results
claim records
blocked claims
phase scope
```

Traces must not embed hidden runtime values. Runtime values belong in payloads.

## 17. Claim Discipline

Every visible report statement must be one of:

```text
ObservedFact
DerivedClaim
BlockedClaim
DeferredClaim
```

Rules:

```text
observed facts require direct support
derived claims require operator path support
blocked claims require named blocker
deferred claims require named future gate / phase / dependency
clinical claims are prohibited
direct NT measurement claims are prohibited
probe-output-alone truth claims are prohibited
```

## 18. Probe Discipline and Overfit Control

Probe results are diagnostic, not truth.

Rules:

```text
frozen probes are baseline
learned probes are not baseline
probe outputs remain separate until explicit aggregation
aggregation creates a new payload and preserves source payload ids
prompt/model sensitivity must be surfaced when confidence depends on stability
```

If learned probes are later introduced, they require:

```text
training data provenance
held-out evaluation
negative/control task
capacity constraint
selectivity report
```

## 19. Snap Constraints

Snap remains the executable and provenance spine.

Respect:

```text
typed nodes
object / operator distinction
directed edge families
weighted edges
dynamic operator-ref weights
canonical deterministic serialization
parse -> serialize -> parse -> serialize identity
```

Do not add new Snap syntax to compensate for weak modeling.

If an edge itself must evolve, reify it as an object / edge record rather than mutating hidden syntax.

## 20. Vibes / Intermediate State Constraints

The intermediate state should support signed and scale-aware Vibes-style state vectors.

The projection layer must validate:

```text
range
diagnostic band
collapse-group logic
scale / frequency role
override conditions
```

Invalid projected states are blocked from prior bridge.

## 21. Current Gaps to Close Before Coding Runtime

### G1. Replace or annotate stale `implementation-plan.snap`

The current file still looks like a fixed alpha/beta/gamma lifecycle.

Required correction:

```text
make phase count variable
rename top-level graph to Resonance implementation plan
treat Alpha/Beta/Gamma as example handles
preserve only if useful as current local handles
```

### G2. Ensure updated `requirements.md` includes the ontology section

The addendum should be folded into `specification/requirements.md` as section 11, not kept only as a detached supplement.

### G3. Create `contracts/readme.md`

This should be the entry point for coding agents.

It should warn:

```text
no broad Kind enums
read contracts before deriving structs
do not flatten roles
do not mix coding instructions with probe declarations
```

### G4. Implement gates before interpretation

Do not build report meaning before gate records exist.

### G5. Implement traces before final outputs

Output cannot bypass trace.

### G6. Keep output directory semantic clear

`output/` means system-level workshop deliverable only.

### G7. Block hidden runtime behavior

Every material dependency and side effect must appear on a declared source-to-sink path.

### G8. Preserve disagreement

Do not merge disagreement into one opaque score.

### G9. Avoid phase-token identity leaks

Phase token can qualify state, not durable identity.

### G10. Keep graph side lightweight

The graph runtime is not the compute bottleneck. Probe inference is the heavier side. Cache probe payloads before graph experiments.

## 22. Developer Agent Work Orders

### Work Order 1 — Requirements Replacement

Replace `specification/requirements.md` with the updated version containing the ontology / contract-alignment section.

Acceptance:

```text
existing requirements preserved
section 11 present
no broad Kind enum rule present
role separation present
phase-token clarification present
```

### Work Order 2 — Contracts Entry Point

Create `contracts/readme.md`.

Acceptance:

```text
lists all 10 contracts
states implementation order
contains minimum questions before coding
contains no broad Kind enum rule
states payload marker and phase suffix rules
```

### Work Order 3 — Contract Completeness Review

Verify these exist:

```text
artifact.md
probe.md
payload.md
operator.md
state.md
requirement.md
gate.md
trace.md
claim.md
output.md
```

Acceptance:

```text
each contract defines role
each contract separates what it is not
each contract has required record or equivalent
each contract has validation/failure rules
```

### Work Order 4 — Implementation Plan Snap Repair

Repair `plan/implementation-plan.snap`.

Acceptance:

```text
project name is Resonance
operators path is ops/resonance/
workspace is ./resonance/
time is absolute UTC with Z
phase-count-is-variable is stated in extras
Alpha/Beta/Gamma are not presented as universal lifecycle
```

### Work Order 5 — Rust Record Skeleton

Implement basic Rust record structs for:

```text
ArtifactRecord
PayloadRecord
OperatorDeclaration
OperatorExecutionRecord
ProbeDeclaration
ProbeExecutionRecord
StateRecord
RequirementRecord
GateDeclaration
GateResult
TraceRecord
ClaimRecord
OutputRecord
```

Acceptance:

```text
no broad Kind enums
ids are newtypes
records contain provenance fields
records do not hide values
payloads reference value_ref
```

### Work Order 6 — Fitness Function Skeleton

Implement fitness function traits / stubs.

Acceptance:

```text
artifact hash stability
payload contract present
operator contracts present
probe provenance complete
state phase-valid
requirement gate-linked
gate declaration complete
trace has sources
claim support present
output source trace present
```

### Work Order 7 — First Totality Harness

Build alpha-style deterministic path.

Acceptance:

```text
one specimen artifact enters
two frozen-probe fixtures or wrappers run
payloads cache
intermediate state payload created
mock prior gain payload created
toy graph transition payload created
trace emitted
phase-local report emitted
```

## 23. Stop Conditions for Developer Agent

Stop and surface instead of guessing if:

```text
a record role is unclear
a contract does not exist
a required input is hidden
a payload value lacks provenance
a gate is missing
a trace cannot link claims to sources
mock data is being used as truth
a broad Kind enum feels tempting
alpha is being judged against gamma requirements
```

## 24. Final Instruction

Do not optimize for cleverness.

Optimize for:

```text
category separation
determinism
traceability
explicit gates
explicit contracts
no hidden state
no fake certainty
small files
declarative graph-first implementation
```

## 25. Active Execution Package

Execute `plan/work-package-01-api.md` now.

Do not widen scope beyond that package.
