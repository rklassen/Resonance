# Resonance Requirements 12

Status: active workshop requirements supplement
Scope: phase, temporary state, output, claim, gate, trace, and probe-validity

This document continues section 11 of `requirements.md` without weakening any
preceding requirement.

## 11.4 Phase Model Clarification

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

## 11.5 Temporary State vs Final Requirement

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

## 11.6 Output vs Payload

The `output/` directory is reserved for system-level workshop deliverables.

Payloads are runtime currency. They may contribute to output, but they are not
automatically output.

Requirements:

1. Local operator `.out` must not be confused with `output/`.
2. Final output must assemble from traces, claims, gate results, and source
   records.
3. Output must not bypass trace.
4. Output must preserve blocked claims and disagreement.

## 11.7 Claim Discipline

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

## 11.8 Gate Discipline Extension

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

## 11.9 Trace Discipline Extension

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

## 11.10 Overfit and Probe-Validity Discipline

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
