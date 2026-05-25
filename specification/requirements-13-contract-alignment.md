# Resonance Requirements 13

Status: active workshop requirements supplement
Scope: contract coverage, requirement alignment, and implementation readiness

This document continues section 11 of `requirements.md` without weakening any
preceding requirement.

## 11.11 Contract Coverage Requirement

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

## 11.12 Requirements-to-Contract Alignment

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

## 11.13 Implementation Agent Rule

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

## 11.14 Acceptance of This Section

This requirements section is accepted when:

1. no preceding requirement is weakened,
2. every major role has a contract,
3. implementation agents have an entry point for category separation,
4. phase tokens remain organizational handles,
5. temporary state cannot masquerade as final truth,
6. outputs cannot bypass traces,
7. probes cannot become opaque truth machines,
8. no broad `Kind` enum pattern is introduced.
