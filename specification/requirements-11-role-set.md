# Resonance Requirements 11

Status: active workshop requirements supplement
Scope: requirement ontology and role separation

This document continues section 11 of `requirements.md` without weakening any
preceding requirement.

## 11.1 Role Set

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

## 11.2 No Broad `Kind` Enum Rule

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

## 11.3 Coding Apparatus vs Evaluator Apparatus

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
