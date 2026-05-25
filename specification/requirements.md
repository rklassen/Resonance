# Resonance Requirements

Status: active workshop requirements

## 1. Purpose

This workshop must reinforce the humanstate metric with visual sentiment
analysis represented as auxiliary chains in a typed directed knowledge graph.
The system must improve causal reasoning about humanstate and context without
pretending to solve the complete graph.

The deliverable is a deterministic inspection machine built around frozen
probes, explicit priors, graph execution, and traceable verification. It is
not a training system, not a clinical instrument, and not a license to invent
unsupported data.

## 2. Product Scope

The product must:

1. ingest an artifact such as an image or text,
2. run one or more frozen probes,
3. project probe outputs into a stable intermediate state,
4. bridge that state into graph-compatible prior fields,
5. execute graph dynamics over a parcel-style runtime,
6. emit a Snap-native trace and verification report.

The product must not require a complete world model before producing useful
output. Sparse shortest-path implementation is acceptable when provenance,
determinism, and declared interfaces remain intact.

## 3. Engineering Principles

The implementation must satisfy these invariants:

1. Declarative first. Data contracts, graph declarations, and typed edges must
   control behavior more than ad hoc imperative glue.
2. Stateless execution. Core compute units must behave as stateless structs or
   pure functions over explicit inputs.
3. Determinism. The same declared inputs, model versions, and runtime options
   must produce the same serialized outputs within declared numeric tolerance.
4. Immutability by default. Records, traces, and cached outputs must be treated
   as append-only or value-replaced, never silently mutated in place.
5. No hidden inputs or effects. Every material dependency and side effect must
   appear on a declared source-to-sink path.
6. No fabricated data. The system must only emit values derived from artifacts,
   frozen probes, declared priors, or explicit mock fixtures used by the active
   phase.
7. Dynamic without brittleness. Extension points must be created through typed
   contracts, manifests, operator refs, and phase-gated replacement, not by
   weakening validation.
8. Runtime humility. Requirements must avoid assuming a specific deployment
   environment beyond what is explicitly declared in code or metadata.
9. Small units. Source files must stay under 400 lines, and components should
   remain narrow enough to validate locally.

## 4. Architectural Requirements

### 4.1 Execution Model

1. The system must be built as a frozen-probe inspection machine.
2. The privileged execution path must be:
   artifact -> frozen probes -> cached latent outputs -> Vibes-style
   projection -> prior bridge -> parcel or highway graph runtime ->
   disagreement and width readout -> Snap trace.
3. Training, fine-tuning, or online adaptation must not be required for the
   baseline product.
4. Cached probe outputs must be reusable across graph experiments.

### 4.2 Snap Spine

1. Snap must remain the executable and provenance spine.
2. Snap support must preserve typed nodes, object and operator distinction,
   directed edges, weighted edges, dynamic operator-ref weights, and canonical
   deterministic serialization.
3. Canonical parse -> serialize -> parse -> serialize must produce identical
   text for supported sections.
4. The system must not introduce new Snap syntax when existing edge families,
   typed nodes, registers, and operator refs can express the same behavior.

### 4.3 Phase Evolution

1. Phase transitions must be explicit and labeled as promoted, wrapped,
   replaced, extended, stress-tested, transformed, removed, or deferred.
2. No implicit upgrade path is allowed.
3. Later phases may extend earlier ones, but gamma behavior must reduce to the
   verified beta substrate when gamma-specific dynamics are disabled.

## 5. Functional Requirements

### 5.1 Artifact Intake

1. The system must load image and text artifacts.
2. The system must compute a stable artifact hash.
3. The system must normalize artifact metadata into a deterministic record.
4. The same artifact must produce the same hash and normalized metadata.

### 5.2 Frozen Probes

1. The system must support at least one embedding-style probe and one
   affective or semantic label probe in the baseline path.
2. Every probe output must be traceable to a model identifier and model hash.
3. Probe execution must be replayable within declared tolerance.
4. Probe outputs must remain separate; the system must not collapse all probe
   evidence into a single opaque score.

### 5.3 Probe Registry and Cache

1. Probe outputs must be cached using a key derived from artifact identity and
   probe identity.
2. Beta and later phases must include model hash, prompt hash when present,
   and preprocessing hash in cache invalidation.
3. A cache hit must skip redundant inference.
4. Cache records must be immutable once written, except for explicit versioned
   replacement.

### 5.4 Intermediate State Projection

1. The system must project probe outputs into a deterministic intermediate
   state representation suitable for graph execution.
2. The baseline intermediate representation must support signed and
   scale-aware Vibes-style state vectors.
3. The projection layer must validate range, collapse-group, and role
   constraints before downstream use.
4. Invalid projected states must be blocked from the prior bridge.

### 5.5 Prior Bridge

1. The system must represent prior-derived gains as explicit records with
   source map and transform provenance.
2. Alpha may use fixed mock mappings.
3. Beta and later phases must align declared priors through an explicit
   transform and parcel mapping process.
4. Prior fields must be identified as priors or gain fields, not as clinical
   measurements.
5. Unsupported or unaligned priors must be blocked rather than guessed.

### 5.6 Graph Runtime

1. The runtime must operate over a declared parcel or highway graph state.
2. Alpha must support a toy sparse graph and deterministic walk.
3. Beta must support a real parcel graph with checked dimensions,
   non-dangling identifiers, and reproducible state transitions.
4. Runtime execution must reject NaN, Inf, and dimension mismatch.
5. Gamma may add directed, phase, and wavelet behavior only if disabling those
   extensions reduces runtime behavior to the verified beta form.

### 5.7 Disagreement and Discovery

1. The system must make disagreement observable rather than hiding it.
2. Disagreement outputs must localize to a probe, prompt, prior, transform,
   operator, or graph edge.
3. Gamma may add discovery surfaces, latent sweeps, and ensemble comparisons,
   but unstable axes must not be promoted to high-confidence claims.

### 5.8 Trace and Reporting

1. Every run must emit a trace containing artifact identity, probe identities,
   projected state, prior bridge output, graph transition, and Snap path.
2. Every visible claim in reports or viewers must link back to a source record,
   prior, Snap edge, or runtime step.
3. Reports must distinguish observed facts, derived claims, blocked claims, and
   deferred claims.
4. The system must emit gate decision states of pass, fail, blocked, or
   deferred against the requirements that are valid for the current phase only.

## 6. Verification Requirements

### 6.1 Truth Vector

1. Verification must evaluate Integration, Performance, and Accuracy.
2. Integration must be derived from completeness, determinism, and
   transparency checks over declared and real graph edges.
3. The composite truth scalar must use geometric mean, not arithmetic mean.
4. A zero on any governing axis is a stop-and-surface event, not a soft score.
5. No axis below Yes may be reported without an explicit named follow-up
   observation that could raise it.
6. When scored ordinally, governing truth axes use the canonical ladder
   Violation, Weak, Mixed, Mostly, Yes.

### 6.2 Gate Discipline

1. Inspection must be phase-gated.
2. Each downstream inspection step must wait until its prerequisite gate is
   satisfied.
3. The system must not judge alpha against gamma-only criteria.
4. The system must not treat shape verification as truth verification.

## 7. Non-Functional Requirements

1. Deterministic serialization and replay are mandatory.
2. All major records must have explicit schemas.
3. Type information must remain available at graph boundaries.
4. Validation failures must be explicit and user-visible.
5. The system must prefer append-only traces and content-addressed records.
6. The graph side should remain lightweight relative to probe inference.
7. Public interfaces must be precise enough to swap mocks for real priors
   without rewriting unrelated layers.

## 8. Out of Scope

The workshop must not:

1. claim a complete causal model of humanstate,
2. fabricate labels, priors, or oracle values,
3. merge all disagreement into one summary score,
4. recompute frozen probes during graph-only experiments when valid cache data
   already exists,
5. present prior fields as direct human measurements,
6. require hidden operator behavior or undocumented runtime state,
7. add Snap syntax to compensate for weak modeling.

## 9. Phase Acceptance

### 9.1 Alpha

Alpha is accepted when one artifact produces a deterministic totality trace
through artifact intake, at least two frozen probes, cache, intermediate state
projection, mock prior gain, toy graph runtime, and Snap-linked reporting.

### 9.2 Beta

Beta is accepted when alpha mocks are replaced or explicitly deferred, real
priors are installed through provenance-bearing transforms, the parcel runtime
is replayable, and reports evaluate beta against beta-valid criteria only.

### 9.3 Gamma

Gamma is accepted when the full probe and prior graph exposes agreement,
disagreement, width, energy, and traceable operator paths without training,
clinical claims, hidden inputs, or manual bottlenecks.

## 10. Implementation Notes

These requirements intentionally specify invariants and acceptance behavior
more strongly than concrete algorithms. Where multiple implementations satisfy
the same determinism, traceability, and verification contracts, the simpler and
more declarative option should be preferred.

## 11. Requirement Ontology and Contract Alignment

This section supplements the active requirements without replacing or weakening
the preceding sections. It defines the front-end category separation needed so
implementation agents do not commingle durable entities, evaluator processes,
runtime currency, lifecycle state, requirements, verification gates, traces,
claims, and workshop outputs.

### 11.1 Role Set

Every declared record, node, edge, process, file, and visible statement must
belong to one primary role:

1. Artifact: source entity inspected by Resonance.
2. Probe: evaluator process, never a coding instruction.
3. Payload: immutable runtime currency.
4. Operator: executable process over declared inputs and outputs.
5. State: lifecycle or phase-local condition, never identity.
6. Requirement: obligation stating what must be true.
7. Gate: executable check for whether proceeding is valid.
8. Trace: provenance record of what happened.
9. Claim: supported, blocked, or deferred reportable statement.
10. Output: system-level workshop deliverable assembled from traceable records.

These roles are independent dimensions. The implementation must not collapse
them for convenience.

### 11.2 No Broad `Kind` Enum Rule

The implementation must not introduce broad semantic bucket enums such as:

```text
ProbeKind
PayloadKind
ArtifactKind
TraceKind
RequirementKind
OutputKind
```

Use contracts, capabilities, policies, lifecycle states, statuses, and gate
decisions instead.

Acceptable examples:

```text
ContractId
CapabilityId
PolicyId
StateLabel
ClaimStatus
GateDecision
RequirementStatus
```

Broad `Kind` enums hide domain semantics and create future migration cost.

### 11.3 Coding Apparatus vs Evaluator Apparatus

Coding-agent instructions and evaluator declarations must remain separate.

Coding-agent instructions belong in:

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

A prompt that tells an agent to write a probe wrapper is not itself a probe.

### 11.4 Phase Model Clarification

Phase tokens such as `alpha`, `beta`, `gamma`, `Α`, `Β`, and `Γ` are cognitive
handles, not a fixed lifecycle.

Requirements:

1. Phase count is variable.
2. A phase exists only when it reduces inference load, integration risk,
   runtime uncertainty, component ambiguity, or verification ambiguity.
3. Trivial cases may use one phase.
4. Complex cases may use many phases.
5. A phase token must not become durable identity.
6. A phase-local requirement must not be applied outside its declared phase.

### 11.5 Temporary State vs Final Requirement

Temporary scaffolds are permitted only when explicit.

Examples:

```text
mock
stub
shape-proof
fixture-backed
exploratory
```

Rules:

1. Temporary states must be phase-scoped.
2. Temporary states may prove shape.
3. Temporary states may not prove truth.
4. Temporary states must have explicit replacement, deferral, removal, or
   promotion path.
5. Reports must disclose temporary state when it affects interpretation.

### 11.6 Output vs Payload

The `output/` directory is reserved for system-level workshop deliverables.

Payloads are runtime currency. They may contribute to output, but they are not
automatically output.

Requirements:

1. Local operator `.out` must not be confused with `output/`.
2. Final output must assemble from traces, claims, gate results, and source
   records.
3. Output must not bypass trace.
4. Output must preserve blocked claims and disagreement.

### 11.7 Claim Discipline

Every visible statement must be represented as a claim record or remain outside
the report.

Claim statuses:

```text
ObservedFact
DerivedClaim
BlockedClaim
DeferredClaim
```

Requirements:

1. Observed facts require direct support.
2. Derived claims require operator path support.
3. Blocked claims require named blocker.
4. Deferred claims require named future gate, phase, or dependency.
5. Clinical claims, direct NT measurement claims, fabricated labels, and final
   truth claims from probe output alone are prohibited.

### 11.8 Gate Discipline Extension

The active requirements already define Integration, Performance, and Accuracy
as verification axes. This section clarifies gate behavior.

Requirements:

1. Gates are executable checks, not prose review.
2. A gate must declare subject contract, prerequisite gates, fitness function,
   phase scope, failure policy, and applicable requirements.
3. A gate result must be Pass, Fail, Blocked, or Deferred.
4. A zero on any governing axis is stop-and-surface.
5. Any axis below `Yes` requires a named follow-up observation.
6. Shape verification must not be treated as truth verification.

### 11.9 Trace Discipline Extension

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

A trace must not embed hidden runtime values. Runtime values belong in payloads.

### 11.10 Overfit and Probe-Validity Discipline

Probe results are diagnostic, not truth.

Requirements:

1. Frozen probes must record model provenance, prompt provenance when present,
   preprocessing provenance, runtime policy, and tolerance policy.
2. Probe outputs must remain separate until an explicit aggregation operator
   creates a new payload.
3. Learned probes are not baseline.
4. If learned probes are introduced, they require training data provenance,
   held-out evaluation, negative/control task, capacity constraint, and
   selectivity report.
5. Prompt sensitivity and cross-model disagreement must be surfaced where
   high-confidence use depends on probe stability.

### 11.11 Contract Coverage Requirement

The following contracts define the minimum front-end separation layer:

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

Implementation agents must read these contracts before deriving final Rust
records, Snap operators, or verification procedures.

### 11.12 Requirements-to-Contract Alignment

Each requirement must map to one or more contracts and one or more gates.

Minimum alignment:

```text
artifact intake -> artifact.md + gate.md
frozen probes -> probe.md + payload.md + gate.md
cache -> payload.md + trace.md + gate.md
state projection -> operator.md + payload.md + state.md
prior bridge -> operator.md + payload.md + trace.md
graph runtime -> operator.md + payload.md + trace.md + gate.md
disagreement -> claim.md + output.md + trace.md
phase evolution -> state.md + requirement.md + gate.md
reporting -> trace.md + claim.md + output.md
```

### 11.13 Implementation Agent Rule

A coding assistant may implement code from these documents, but must not invent
new category shortcuts.

Before implementing a module, an agent must answer:

```text
What role is this?
What contract governs it?
What inputs are declared?
What outputs are declared?
What gate verifies it?
What trace records it?
What phase scope applies?
Can it support claims?
```

If any answer is missing, the module is not ready.

### 11.14 Acceptance of This Section

This requirements section is accepted when:

1. no preceding requirement is weakened,
2. every major role has a contract,
3. implementation agents have an entry point for category separation,
4. phase tokens remain organizational handles,
5. temporary state cannot masquerade as final truth,
6. outputs cannot bypass traces,
7. probes cannot become opaque truth machines,
8. no broad `Kind` enum pattern is introduced.

