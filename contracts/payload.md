Below is a tight replacement summary for contracts/payload.md. It should describe only the payload contract, not the whole Resonance plan.

# Payload Contract
Status: active contract  
Scope: Resonance runtime currency and immutable process outputs
## Purpose
A `Payload` is immutable runtime currency: a value emitted by one declared process and consumed by another.
Payloads are not artifacts, probes, processes, requirements, gates, claims, phase states, or final workshop outputs. They are the values moving through Resonance: embeddings, logits, label responses, Vibes vectors, prior-gain tables, graph states, trajectories, disagreement maps, and trace fragments.
This separation follows the provenance distinction between activities and the entities they generate: a process produces a value, and the value carries provenance back to the activity and source entities that generated it.  [oai_citation:0‡W3C](https://www.w3.org/TR/prov-dm/?utm_source=chatgpt.com)
## Core Rule
A payload is defined by:
```text
producer execution
source artifacts / source payloads
payload contract
value reference
value hash where required
numeric policy where required
provenance policy
phase scope when temporary

It is not defined by a kind enum.

Required Record

pub struct PayloadRecord {
    pub payload_id: PayloadId,
    pub payload_contract: ContractId,
    pub producer_execution_id: ExecutionId,
    pub source_artifact_ids: Vec<ArtifactId>,
    pub source_payload_ids: Vec<PayloadId>,
    pub value_ref: ValueRef,
    pub value_hash: Option<HashDigest>,
    pub numeric_policy: Option<NumericPolicyId>,
    pub provenance_policy: ProvenancePolicyId,
    pub phase_scope: Option<PhaseToken>,
    pub created_at_utc: String,
}

Contract-First Semantics

Payloads use contracts, not fixed categories.

Examples:

contract.embedding.f32-vector
contract.logits.f32-vector
contract.labels.string-list
contract.vibes-state.signed-vector
contract.prior-gain.parcel-table
contract.parcel-state.f32-vector
contract.graph-trajectory.series
contract.disagreement.map
contract.width-energy.readout
contract.trace.fragment

Downstream operators must check contract compatibility before consuming a payload. Contract mismatch blocks execution.

Value Reference

value_ref points to where the value is stored.

Examples:

inline://small-json
parquet://cache/probes/part-0001.parquet#row=42
zarr://cache/tensors/probe.zarr/group/path
file://output/traces/run-001.json
snap://trace/run-001#node=p301

Large arrays must not be embedded directly in Snap graph declarations. Hidden storage paths are prohibited.

Value Hash

value_hash is required for cacheable or replay-compared payloads.

A changed value requires a new payload record. Content-addressable storage is useful here because content-derived identifiers make immutable computation records easier to replay and verify.  ￼

Numeric Policy

Numeric payloads must declare a numeric policy.

Examples:

f32-vector.cosine-delta-1e-4
f32-vector.abs-delta-1e-5
snorm-vector.range-minus1-plus1
unorm-confidence.range-0-plus1
top-k.rank-stable-5

Baseline rule: reject NaN and Inf.

Valid Transitions

Allowed:

ProbeExecution -> Payload
Payload -> IntermediateStateProjection
Payload -> PriorBridge
Payload -> GraphRuntime
Payload -> TraceAssembly
Payload -> ReportAssembly

Disallowed:

Payload -> clinical claim
Payload -> direct final truth
Payload -> hidden aggregate
Payload -> mutable scratch state

Correct route:

Payload
→ declared operator
→ new Payload
→ Trace
→ Output

Temporary Payloads

Temporary or mock payloads are allowed only when explicitly phase-scoped.

They must declare:

phase_scope
mock_policy
producer_execution_id
source explanation
blocked-from-final-claim flag

Mock payloads may prove shape. They may not prove truth.

Aggregation

Aggregation creates a new payload and must preserve:

input_payload_ids
aggregation_operator_id
weights or rules
confidence
conflict
unsupported mass

Aggregation must not erase the original payloads or hide disagreement.

Snap Naming

Use ❇ for payload/view names:

Probe❇embedding
Probe❇logits
VibesProjection❇state
PriorBridge❇gain-table
GraphRuntime❇trajectory
Readout❇disagreement

Use phase suffixes only when the payload record itself is phase-scoped:

Payload-Α.mock
Payload-Β.verified

Verification Gates

A payload is valid only if:

1. it declares a payload contract,
2. it references a producing execution,
3. it references source artifacts, source payloads, or explicit mock policy,
4. its value_ref resolves,
5. cacheable or replay-compared values declare value_hash,
6. numeric payloads satisfy numeric policy,
7. NaN and Inf are rejected,
8. mocks are blocked from final truth claims,
9. aggregations preserve source payloads,
10. payloads are not confused with artifacts, processes, or outputs.

Acceptance

The payload contract is accepted when payloads are immutable, contract-validated, provenance-bearing runtime values that can be replayed, traced, blocked, aggregated, or promoted without collapsing into hidden state or unsupported claims.
