Implementation plan alpha-beta

5. Alpha implementation

Alpha goal

First totality output:

artifact → 2 probes → Vibes vector → mock receptor gain → mock 360 graph → toy walk → trace

Alpha modules

A1 — Snap-Spine-Α

Implement subset:

.graph
nodes
edges
registers
types

Observe: valid .snap.

Verify: roundtrip stable.

⸻

A2 — Artifact-Intake-Α

Implement:

load_image()
load_text()
hash_artifact()
normalize_metadata()

Observe: artifact_hash.

Verify: same artifact, same hash.

⸻

A3 — Frozen-Probe-Α

Implement two probe wrappers:

1. image/text embedding probe,
2. affect/semantic label probe.

Observe: embeddings/logits.

Verify: deterministic replay.

⸻

A4 — Probe-Cache-Α

Implement:

cache_key = artifact_hash + model_id + prompt_id

Store vectors in Parquet or Zarr.

Observe: cache hit/miss.

Verify: second run skips inference.

⸻

A5 — Vibes-Projection-Α

Implement deterministic mapping:

probe outputs → signed 11D/12D Vibes vector

Observe: Vibes vector.

Verify: ranges, collapse-group logic, scale roles.

⸻

A6 — Receptor-Gain-Α

Use mock fixed mapping.

Observe: gain vector.

Verify: deterministic output.

Do not compare to real receptor atlas yet.

⸻

A7 — Parcel-Graph-Α

Create mock graph:

360 nodes
toy sparse adjacency
toy Laplacian

Observe: adjacency, degree vector, Laplacian.

Verify: dimensions and sparsity.

⸻

A8 — Laplacian-Walk-Α

Implement:

state_next = state - η L state + gain

Observe: parcel perturbation.

Verify: deterministic transition.

⸻

A9 — Trace-Report-Α

Emit:

artifact_hash
probe ids
Vibes vector
mock gain
graph transition
Snap path

Alpha acceptance: end-to-end trace exists and is deterministic.

⸻

6. Beta implementation

Beta goal

Replace mocks with real priors and make the privileged path correct.

artifact → frozen probes → cache → Vibes → real receptor bridge → Glasser/360 graph → Laplacian runtime → disagreement/width-v1

Beta integration from alpha

Beta component	Alpha source	Action
Snap-Spine-Β	Snap-Spine-Α	extend
Probe-Registry-Β	Frozen-Probe-Α	wrap
Probe-Cache-Β	Probe-Cache-Α	wrap
Vibes-Projection-Β	Vibes-Projection-Α	promote
Receptor-Bridge-Β	Receptor-Gain-Α	replace
Parcel-Graph-Β	Parcel-Graph-Α	replace
Laplacian-Runtime-Β	Laplacian-Walk-Α	replace
Disagreement-Width-Β	Trace-Report-Α	extend
Requirements-Report-Β	Alpha reports	wrap

⸻

B1 — Snap-Spine-Β

Complete all Snap sections.

Add dynamic operator refs:

Vibes-Projection-Β -(@receptor-gain-op)s-> Receptor-Bridge-Β

Verify: operator refs resolve, runtime can evaluate dynamic weights.

⸻

B2 — Probe-Registry-Β

Implement model manifest:

model_id
model_hash
provider
input_shape
output_schema
prompt_schema

Verify: every probe output links to a model manifest.

⸻

B3 — Probe-Cache-Β

Upgrade cache invalidation:

artifact_hash + model_hash + prompt_hash + preprocessing_hash

Verify: changing any dependency invalidates cache.

⸻

B4 — Vibes-Projection-Β

Add verifier:

range check
band check
collapse-group check
scale/frequency role check

Verify: no Vibes vector enters receptor bridge without validation.

⸻

B5 — Receptor-Bridge-Β

Install:

1. receptor map loader,
2. neuromaps transform,
3. parcel alignment,
4. provenance record.

Verify:

source map → coordinate transform → parcel table

has provenance and dimension checks.

⸻

B6 — Parcel-Graph-Β

Install:

360 parcel ids
hemisphere labels
highway edge schema
sparse adjacency
Laplacian

Verify:

360 nodes
edge count > 0
no dangling ids
L shape = 360×360 or block-decomposed equivalent

⸻

B7 — Laplacian-Runtime-Β

Implement graph-neural-field-compatible runtime:

state_t+1 = f(L, state_t, receptor_gain, local_dynamics)

Verify:

1. replay deterministic,
2. stability bounds,
3. no NaN/Inf,
4. graph dimensions match.

⸻

B8 — Disagreement-Width-Β

Implement:

probe_disagreement
receptor_projection_disagreement
graph_spread
energy_proxy
unsupported_edges

Verify: every disagreement names source probes/priors/operators.

⸻

B9 — Requirements-Report-Β

Emit pass/fail matrix.

Verify: beta does not judge itself against gamma-only criteria.

`#end resonance/implementation-plan-alpha-beta.rs`