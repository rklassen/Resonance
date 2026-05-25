# Operator Contract

Status: active contract  
Scope: Resonance executable processes, transformations, evaluators, and runtime actions

## Purpose

An `Operator` is a declared executable process.

It consumes artifacts or payloads through explicit input contracts, performs one stateless operation, and emits payloads, traces, gate results, or phase transition records through explicit output contracts.

A probe is a specialized operator. A verifier is a specialized operator. A prior bridge, graph runtime, projector, aggregator, and report assembler are also operators.

An operator is not a payload, artifact, requirement, gate result, claim, final output, or coding-agent instruction.

## Core Rule

An operator is defined by:

```text
operator id
input contracts
output contracts
declared capability
runtime policy
side-effect policy
determinism policy
failure policy
```

It is not defined by a `kind` enum.

## Operator vs Coding Instruction

A coding-agent instruction tells a developer or agent how to build code.

An operator is the code-facing executable declaration that the runtime may invoke.

Do not store developer prompts, TODOs, or implementation instructions as operators.

## Required Declaration

```rust
pub struct OperatorDeclaration {
    pub operator_id: OperatorId,
    pub display_name: String,

    pub input_contracts: Vec<ContractId>,
    pub output_contracts: Vec<ContractId>,
    pub declared_capabilities: Vec<CapabilityId>,

    pub runtime_policy: RuntimePolicyId,
    pub determinism_policy: DeterminismPolicyId,
    pub side_effect_policy: SideEffectPolicyId,
    pub failure_policy: FailurePolicyId,

    pub phase_scope: Option<PhaseToken>,
}
```

## Required Execution Record

```rust
pub struct OperatorExecutionRecord {
    pub execution_id: ExecutionId,
    pub operator_id: OperatorId,

    pub input_artifact_ids: Vec<ArtifactId>,
    pub input_payload_ids: Vec<PayloadId>,

    pub output_payload_ids: Vec<PayloadId>,
    pub output_gate_result_ids: Vec<GateResultId>,
    pub output_trace_ids: Vec<TraceId>,

    pub runtime_policy: RuntimePolicyId,
    pub created_at_utc: String,
}
```

## Stateless Execution

Core operators must behave as stateless structs or pure functions over explicit inputs.

Rules:

1. Every material input must be declared.
2. Hidden global state is prohibited.
3. Environment-dependent behavior must be captured in runtime policy.
4. Randomness requires declared seed and policy.
5. Side effects must be declared before execution.

## Side Effects

Baseline allowed side effects:

```text
write-cache-record
write-trace-record
write-output-file
read-declared-model
read-declared-prior
```

Prohibited side effects:

```text
hidden-network-call
undeclared-file-read
silent-cache-mutation
model-weight-update
online-adaptation
clinical-action
```

## Dynamic Operator Refs

Snap dynamic operator-ref weights may reference operator nodes when the output contract matches the expected weight/payload contract.

Rules:

1. Operator refs must resolve to declared operators.
2. Operator output contract must be compatible with the edge.
3. Dynamic weight output must obey declared numeric policy.
4. Operator-ref behavior must be replayable.
5. Operator refs must not weaken validation.

## Valid Transitions

Allowed:

```text
Artifact -> OperatorExecution -> Payload
Payload -> OperatorExecution -> Payload
Payload -> OperatorExecution -> GateResult
GateResult -> OperatorExecution -> Trace
```

Disallowed:

```text
OperatorExecution -> hidden state
OperatorExecution -> unsupported claim
OperatorExecution -> clinical action
OperatorExecution -> direct final truth
```

## Snap Encoding

Operators should map directly to Snap `operator` or `function` nodes.

```snap
operator {
 id: o201,
 in: { input: Payload, },
 name: 'project-payload-to-vibes-state',
 out: { output: Payload, },
}
```

Operator outputs use payload records rather than hidden values.

```text
Operator❇execution-record
Operator❇output-payload
Operator❇gate-result
```

Phase suffixes describe lifecycle state of an operator declaration:

```text
Operator-Α.mock
Operator-Β.verified
Operator-Γ.extended
```

## Verification Gates

An operator declaration is valid only if:

1. input contracts are declared,
2. output contracts are declared,
3. runtime policy is declared,
4. determinism policy is declared,
5. side-effect policy is declared,
6. failure policy is declared,
7. all side effects are explicit,
8. outputs are records, payloads, gate results, or traces,
9. no hidden adaptation occurs,
10. operator refs resolve and type-check.

## Failure Modes

Block operator execution or promotion when:

1. input contract missing,
2. output contract missing,
3. hidden input detected,
4. undeclared side effect detected,
5. runtime policy missing,
6. output contract mismatch,
7. NaN or Inf emitted where prohibited,
8. dynamic operator ref unresolved,
9. operator mutates cached records silently,
10. operator emits unsupported final claim.

## Rust Sketch

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OperatorId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CapabilityId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DeterminismPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SideEffectPolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FailurePolicyId(pub String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorDeclaration {
    pub operator_id: OperatorId,
    pub display_name: String,
    pub input_contracts: Vec<ContractId>,
    pub output_contracts: Vec<ContractId>,
    pub declared_capabilities: Vec<CapabilityId>,
    pub runtime_policy: RuntimePolicyId,
    pub determinism_policy: DeterminismPolicyId,
    pub side_effect_policy: SideEffectPolicyId,
    pub failure_policy: FailurePolicyId,
    pub phase_scope: Option<PhaseToken>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorExecutionRecord {
    pub execution_id: ExecutionId,
    pub operator_id: OperatorId,
    pub input_artifact_ids: Vec<ArtifactId>,
    pub input_payload_ids: Vec<PayloadId>,
    pub output_payload_ids: Vec<PayloadId>,
    pub output_gate_result_ids: Vec<GateResultId>,
    pub output_trace_ids: Vec<TraceId>,
    pub runtime_policy: RuntimePolicyId,
    pub created_at_utc: String,
}
```

## Acceptance

The operator contract is accepted when every executable process has explicit inputs, outputs, policies, side effects, execution records, and validation gates, without hidden state or semantic buckets.
