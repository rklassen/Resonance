# Contracts Index

Status: active implementation entry point  
Scope: `contracts/` folder

## Purpose

This folder defines the front-end category separation layer for Resonance.

These contracts prevent implementation agents from commingling source entities,
evaluator processes, runtime currency, lifecycle state, requirements, gates,
traces, claims, and final workshop outputs.

## Rule Zero

Do not introduce broad `Kind` enums.

Use:

```text
contracts
capabilities
policies
lifecycle states
statuses
gate decisions
```

Do not use:

```text
ProbeKind
PayloadKind
ArtifactKind
TraceKind
RequirementKind
OutputKind
```

## Contract Set

### `artifact.md`

Defines source entities inspected by Resonance.

Use when implementing artifact intake, hashing, normalization, and artifact
metadata.

Artifact is not payload, process, output, or claim.

### `probe.md`

Defines evaluator processes.

Use when implementing frozen model wrappers, deterministic evaluators, prompt
policies, probe manifests, and probe execution records.

Probe is not coding instruction and not emitted value.

### `payload.md`

Defines immutable runtime currency.

Use when implementing embeddings, logits, labels, Vibes states, prior-gain
tables, graph states, trajectories, disagreement maps, and trace fragments.

Payload is not artifact, process, trace, or final output.

### `operator.md`

Defines executable processes.

Use when implementing projectors, prior bridges, graph runtimes, aggregators,
verifiers, report assemblers, and probe wrappers.

Operator is not a developer instruction.

### `state.md`

Defines lifecycle and phase-local state.

Use when implementing phase scopes, mocks, stubs, verified states, blocked
states, deferred states, and explicit mutation verbs.

State is not identity and not payload value.

### `requirement.md`

Defines obligations.

Use when implementing requirement records, requirement status, phase
applicability, and requirement-to-gate mapping.

Requirement is not gate.

### `gate.md`

Defines executable readiness and verification checks.

Use when implementing pass/fail/blocked/deferred decisions, truth axes,
geometric mean, stop-and-surface behavior, and follow-up observations.

Gate is not requirement.

### `trace.md`

Defines provenance records.

Use when implementing run traces, trace steps, Snap paths, gate result links,
claim support links, and replay evidence.

Trace records what happened; it does not hide payload values.

### `claim.md`

Defines reportable statements.

Use when implementing observed facts, derived claims, blocked claims, deferred
claims, uncertainty, blockers, and support links.

Claim is not truth by default.

### `output.md`

Defines system-level workshop deliverables.

Use when implementing files in `output/`, reports, exports, review summaries,
and discovery surfaces.

Output is not local operator `.out`.

## Implementation Order

Recommended implementation order:

```text
artifact
payload
operator
probe
state
requirement
gate
trace
claim
output
```

Reason:

1. source identity first,
2. runtime currency second,
3. executable processes third,
4. evaluators as specialized processes fourth,
5. lifecycle state before requirements/gates,
6. gates before trace/report interpretation,
7. claims and outputs last.

## Minimum Questions Before Coding

For every new Rust struct, Snap node, or procedure, answer:

```text
What role is this?
Which contract governs it?
What are its declared inputs?
What are its declared outputs?
Is it durable, temporary, or phase-scoped?
What gate verifies it?
What trace records it?
Can it support a claim?
Can it appear in final output?
```

If the answers are unclear, do not implement yet.

## Phase Rule

Phase tokens are cognitive handles.

They are not a fixed lifecycle and must not be durable identity.

Examples:

```text
-Α
-Β
-Γ
```

Use more or fewer phases as needed.

## Payload Marker Rule

Use `❇` for emitted payloads and views:

```text
Probe❇embedding
VibesProjection❇state
Readout❇disagreement
Output❇summary
```

Use phase suffixes only for lifecycle state:

```text
Probe-Β.verified
Payload-Α.mock
Trace-Γ.discovery-run
```

## Claim Safety

No output may include an unsupported claim.

Every visible statement must be one of:

```text
ObservedFact
DerivedClaim
BlockedClaim
DeferredClaim
```

## Stop Conditions

Stop and surface when:

```text
hidden input appears
provenance is missing
contract mismatch occurs
phase-inappropriate inspection occurs
mock is used as truth
NaN or Inf appears in numeric runtime
zero governing truth axis occurs
probe output is treated as final truth
```

## Acceptance

The contracts folder is complete enough for implementation when:

1. every major role has a document,
2. no role depends on a broad `Kind` enum,
3. implementation agents can determine where each new struct belongs,
4. claims cannot bypass trace,
5. output cannot bypass gates,
6. probes cannot bypass payload records,
7. temporary state cannot masquerade as final truth.
