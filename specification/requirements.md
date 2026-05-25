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
9. Small units. Files must satisfy the applicable repo line cap after normal
   formatter output. The workshop target remains under 400 lines, except for
   the stable Snap DSL reference at `specification/snap-spec-0.8.md`, which
   remains whole by explicit exception. Rust source files must in all cases
   remain within the locked repo cap.
10. Release-first validation. Rust validation and executable checks must run
    in `--release` by default unless the active task is explicitly debugging.

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
   replaced, extended, stress-tested, or removed.
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

This section remains active, but it is split into flat companion documents so
the requirements stay complete without compressing content or creating nested
document trees.

Read these in order:

1. `requirements-11-role-set.md`
2. `requirements-12-phase-claim-trace.md`
3. `requirements-13-contract-alignment.md`

