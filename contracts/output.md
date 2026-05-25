# Output Contract

Status: active contract  
Scope: Resonance system-level workshop deliverables

## Purpose

An `Output` is a system-level workshop deliverable produced after declared execution and verification.

Output is not a local operator `.out`, not a payload, not a trace, not a process, not a requirement, and not a gate. It may contain or reference traces, reports, exports, Snap files, and rendered summaries, but it must remain distinct from component-local outputs.

## Core Rule

An output is defined by:

```text
output id
input trace ids
included claim ids
included gate result ids
export format
phase scope
audience or use context
generation operator
```

It is not defined by a `kind` enum.

## Output vs Payload

Payload:

```text
runtime currency between operators
```

Output:

```text
system-level deliverable for the workshop/user/environment
```

Payloads may contribute to output. They are not automatically output.

## Output vs Trace

Trace:

```text
provenance record of what occurred
```

Output:

```text
rendered or exported deliverable assembled from traces and other records
```

## Required Record

```rust
pub struct OutputRecord {
    pub output_id: OutputId,
    pub display_name: String,

    pub source_trace_ids: Vec<TraceId>,
    pub included_claim_ids: Vec<ClaimRecordId>,
    pub included_gate_result_ids: Vec<GateResultId>,

    pub export_ref: ValueRef,
    pub generation_operator_id: OperatorId,

    pub phase_scope: Option<PhaseToken>,
    pub created_at_utc: String,
}
```

## Allowed Output Forms

Examples:

```text
report.md
trace.json
graph.snap
viewer-export.html
requirements-review.md
phase-pass-fail.md
disagreement-map.json
```

Rules:

1. Output form must be declared.
2. Output must not hide trace/gate/claim provenance.
3. Output must not merge disagreement into one opaque score.
4. Output must distinguish observed facts, derived claims, blocked claims, and deferred claims.
5. Output must state active phase scope when phase-relevant.

## System-Level Directory

Use `output/` only for system-level deliverables.

Recommended structure:

```text
output/
  traces/
  reports/
  exports/
  reviews/
```

Do not use `output/` to mean a local operator port or payload emission.

## Claim Inclusion

Every visible claim in an output must reference a claim record.

Rules:

1. Observed facts must reference source records or gate results.
2. Derived claims must reference payloads and operators.
3. Blocked claims must name blockers.
4. Deferred claims must name later gates or phases.
5. Unsupported claims are prohibited.

## Phase-Local Reporting

An output must evaluate only requirements valid for its active phase.

Rules:

1. Alpha output may prove totality shape.
2. Alpha output must not claim final truth.
3. Beta output may prove privileged-path correctness.
4. Gamma output may expose discovery apparatus behavior.
5. Output must not judge one phase against later-phase-only requirements.

## Valid Transitions

Allowed:

```text
Trace -> Output
GateResult -> Output
ClaimRecord -> Output
Payload -> ReportAssembly -> Output
```

Disallowed:

```text
Payload -> direct final output without trace
ProbeExecution -> direct output claim
MockPayload -> truth output
Output -> hidden runtime state
```

## Snap Encoding

Output records may be object nodes:

```snap
object {
 id: u201,
 name: 'Output-Β.requirements-review',
 type: OutputRecord,
}
```

Output views use `❇` when referring to fragments:

```text
Output❇summary
Output❇blocked-claims
Output❇gate-table
Output❇snap-export
```

Phase suffixes indicate output lifecycle scope:

```text
Output-Α.shape-report
Output-Β.privileged-path-report
Output-Γ.discovery-surface
```

## Verification Gates

An output is valid only if:

1. it references source traces,
2. it references included claims,
3. it references included gate results,
4. it declares export ref,
5. it declares generation operator,
6. visible claims are classified,
7. active phase scope is clear when phase-relevant,
8. blocked/deferred claims are preserved,
9. disagreement is not erased,
10. output path does not bypass trace.

## Failure Modes

Block output acceptance when:

1. output lacks source trace,
2. output includes unclassified claims,
3. output hides failed gates,
4. output collapses disagreement into one score,
5. output claims clinical or complete causal truth,
6. output uses mock data as truth,
7. output lacks generation operator,
8. output confuses payload with final deliverable,
9. output evaluates against wrong phase requirements.

## Rust Sketch

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OutputId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputRecord {
    pub output_id: OutputId,
    pub display_name: String,
    pub source_trace_ids: Vec<TraceId>,
    pub included_claim_ids: Vec<ClaimRecordId>,
    pub included_gate_result_ids: Vec<GateResultId>,
    pub export_ref: ValueRef,
    pub generation_operator_id: OperatorId,
    pub phase_scope: Option<PhaseToken>,
    pub created_at_utc: String,
}
```

## Acceptance

The output contract is accepted when workshop deliverables are assembled only from traceable records, preserve disagreement and blocked claims, remain phase-local, and never confuse local operator outputs or payloads with final system-level deliverables.
