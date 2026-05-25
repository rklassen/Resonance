Complete implementation plan

Fixed architecture rule

Build this as a frozen-probe inspection machine, not a training system:

artifact
→ frozen probes
→ cached latent outputs
→ Vibes projection
→ receptor/NT prior bridge
→ parcel/highway graph
→ Laplacian / magnetic-wavelet runtime
→ disagreement + width readout
→ Snap trace

Snap remains the executable/provenance spine: typed nodes, object/operator distinction, directed edges, weighted edges, dynamic operator-ref weights, and deterministic serialization are already in the spec.  ￼

⸻

1. Repository structure

visant/
  crates/
    snap-core/
    snap-runtime/
    graph-runtime/
    probe-runtime/
  py/
    priors/
    neuromaps_bridge/
    receptor_bridge/
  data/
    priors/
    cache/
    traces/
  specs/
    snap/
    phases/
  apps/
    cli/
    viewer/

Reasoning: Rust owns deterministic graph execution and Snap serialization. Python owns neuroscience tooling because neuromaps, atlas handling, and model-conversion ecosystem are Python-first. Zarr is used for n-dimensional arrays because it supports chunking and compression.  ￼

⸻

2. Phase suffix convention

Use suffixes, dashes, and quoted semantic names:

Snap-Spine-Α
Probe-Cache-Β
Discovery-Surface-Γ
ProbeCache❇logits

Rules:

1. Α / Β / Γ = lifecycle phase of the component.
2. ❇ = emitted payload/view, not component lifecycle state.
3. Snap ids stay ASCII: a101, b103, g109.
4. Dashes live in name: strings, not bare ids.

⸻

3. Core data contracts

3.1 Artifact record

ArtifactRecord {
  artifact_id: String,
  artifact_hash: String,
  kind: Image | Text | Snap | Source,
  path: String,
  normalized_metadata: Map,
}

Verified when: same file gives same hash and metadata.

⸻

3.2 Probe record

ProbeRecord {
  artifact_hash: String,
  model_id: String,
  model_hash: String,
  prompt_id: Option<String>,
  output_kind: Embedding | Logits | Labels,
  vector_ref: String,
  created_at: String,
}

Verified when: same artifact/model/prompt gives same output within tolerance.

Use ONNX Runtime + DirectML on Windows because DirectML accelerates ONNX models on commodity GPUs without vendor-specific extensions.  ￼ Microsoft also documents DirectML as an ONNX Runtime execution provider alongside CPU/CUDA/TensorRT-style backends.  ￼

⸻

3.3 Vibes state record

VibesState {
  state_11d: [f32; 11],
  state_12d: [f32; 12],
  collapse_flags: Vec<String>,
  scale_roles: Vec<String>,
  source_probe_ids: Vec<String>,
}

This is grounded in the uploaded Vibes encoder: 8D NT basis, 11D signed state, collapse groups, 12D scale-aware encoder, alignment/distance, and graph propagation.  ￼

⸻

3.4 Receptor bridge record

ReceptorGain {
  parcel_id: String,
  receptor_family: String,
  gain: f32,
  source_map: String,
  transform_id: String,
}

neuromaps is the transform/comparison layer because it provides access, transformation, and analysis of structural, functional, molecular, and other brain annotations across coordinate systems.  ￼

⸻

3.5 Graph state record

ParcelState {
  parcel_id: String,
  hemisphere: L | R,
  value: f32,
  phase: Option<f32>,
  frequency_band: Option<String>,
  timestep: u32,
}

Graph neural fields justify the connectome-Laplacian runtime because they use the graph Laplacian to implement neural activity models directly on the human connectome.  ￼

⸻

4. Snap implementation tasks

4.1 Parser / serializer

Implement or complete:

1. .graph
2. nodes
3. edges
4. extras
5. layout
6. literals
7. registers
8. streams
9. types

Acceptance:

parse → canonical serialize → parse → canonical serialize

must produce identical text.

Snap requires deterministic section order and canonical serialized output; types remains last.  ￼

⸻

4.2 Weighted edge parser

Support:

a -(0.5)s-> b
a -(0.875)u-> b
a -(@operator_id)s-> b
a -(@stream_id ..1024)s-> b

Acceptance:

1. snorm validates [-1, 1].
2. unorm validates [0, 1].
3. operator refs resolve to operator.
4. stream refs require slice syntax.

Snap v0.8 already defines numerical edge weights and dynamic operator-ref weights.  ￼

⸻

4.3 Evolution edge families

Add no new syntax. Use edge families:

edges {
 promote { ... }
 wrap { ... }
 replace { ... }
 extend { ... }
 stress-test { ... }
}

Acceptance: every phase transition is one of:

promoted | wrapped | replaced | extended | stress-tested | removed

No implicit upgrades.

⸻

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

neuromaps includes curated annotations and four standard coordinate systems, making it the right bridge for molecular/functional/structural map alignment.  ￼

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

⸻

7. Gamma implementation

Gamma goal

Full discovery apparatus over frozen encoders and public priors.

full probe suite
→ latent sweeps
→ prior ensemble
→ objective/narrative dual path
→ magnetic/wavelet runtime
→ cross-projection readout
→ discovery surface

Gamma integration from beta

Gamma component	Beta source	Action
Probe-Suite-Γ	Probe-Registry-Β	extend
Latent-Axis-Sweep-Γ	Probe-Cache-Β	extend
Probe-Validity-Evaluator-Γ	Disagreement-Width-Β	wrap
Prior-Ensemble-Γ	Receptor-Bridge-Β	extend
Receptor-Bridge-Γ	Receptor-Bridge-Β	extend
Dual-Path-Runtime-Γ	Parcel-Graph-Β	extend
Magnetic-Wavelet-Runtime-Γ	Laplacian-Runtime-Β	replace/extend
Cross-Projection-Readout-Γ	Disagreement-Width-Β	extend
Discovery-Surface-Γ	Requirements-Report-Β	wrap

Gamma component status

Legend:

✅ complete and verified
🔨 under construction
🔍 status unknown from current repo evidence
⚠️ known gap that must be finished before gamma is complete
❌ implemented but non-compliant and requires correction

Current status:

✅ G1 — Probe-Suite-Γ
Verified by `run_gamma_probe_suite()` and `gamma_reduces_to_beta_when_extensions_disabled()`.

✅ G2 — Latent-Axis-Sweep-Γ
Verified by the fixture-backed sweep test and direct branch tests for stable-left, stable-right, and spread-gated unstable classification.

✅ G3 — Probe-Validity-Evaluator-Γ
Verified by the gamma probe-validity evaluator and the unstable-axis promotion guard test.

✅ G4 — Prior-Ensemble-Γ
Verified by the gamma prior-ensemble observable and the aligned-or-blocked prior test.

✅ G5 — Receptor-Bridge-Γ
Verified by the gamma receptor-bridge ensemble observable and the provenanced family comparison test.

⚠️ G6 — Dual-Path-Runtime-Γ
Planned, but objective and narrative path separation is not implemented yet.

⚠️ G7 — Magnetic-Wavelet-Runtime-Γ
Planned, but phase/wavelet runtime extensions and the beta-reduction invariant are not implemented yet.

⚠️ G8 — Cross-Projection-Readout-Γ
Planned, but cross-projection disagreement localization is not implemented yet.

⚠️ G9 — Discovery-Surface-Γ
Planned, but the gamma viewer/export surface and visible-claim trace links are not implemented yet.

⸻

G1 — Probe-Suite-Γ

Add probe families:

1. visual-semantic,
2. affect/emotion,
3. aesthetic,
4. material/scene,
5. color harmony/context,
6. compatibility/harmonization.

Verify: each probe family has model id, output schema, and cache support.

⸻

G2 — Latent-Axis-Sweep-Γ

Implement zero-shot/prompt sweeps:

cheap ↔ premium
organic ↔ synthetic
calm ↔ loud
earthy ↔ neon
threat ↔ safety
approach ↔ avoidance

Verify: axes are stable across prompt variants or marked unstable.

⸻

G3 — Probe-Validity-Evaluator-Γ

Implement failure modes:

prompt sensitivity
model disagreement
embedding-neighborhood instability
label collision
domain mismatch

Verify: unstable axes cannot promote to high-confidence claims.

⸻

G4 — Prior-Ensemble-Γ

Add multiple priors:

receptor maps
functional gradients
structural connectivity
visual benchmark priors
imagery priors

Verify: every prior is coordinate-aligned or explicitly blocked.

⸻

G5 — Receptor-Bridge-Γ

Extend beta receptor bridge to support ensemble comparisons:

gain mean
gain variance
source disagreement
unsupported receptor family

Verify: no gain field without provenance.

⸻

G6 — Dual-Path-Runtime-Γ

Split:

objective path:
  perceptual/artifact features → parcel graph
narrative path:
  semantic/affect probes → Vibes/receptor bridge

Verify: each path independently produces trace before comparison.

⸻

G7 — Magnetic-Wavelet-Runtime-Γ

Implement:

directed/phase edge terms
frequency-scale wavelet filtering
recirculation operator

Verification invariant:

phase disabled → beta Laplacian runtime

This prevents gamma from drifting away from the verified beta substrate.

⸻

G8 — Cross-Projection-Readout-Γ

Implement:

visual ↔ semantic agreement
semantic ↔ Vibes agreement
Vibes ↔ receptor agreement
receptor ↔ parcel agreement
parcel ↔ graph trajectory agreement

Verify: every disagreement localizes to one of:

probe
prompt
prior
transform
operator
graph edge

⸻

G9 — Discovery-Surface-Γ

Implement viewer/export:

Snap graph
DAG trace
probe matrix
Vibes vector
receptor table
parcel trajectory
disagreement graph
width/energy readout

Verify: every visible claim links back to artifact hash, probe record, Snap edge, prior, and runtime step.

⸻

8. Verification gates

Do not inspect before the gate is ready.

Gate	Inspect only after	Allowed inspection
O1	Snap roundtrip	syntax/canonical form
O2	artifact hash	intake determinism
O3	probe cache	probe repeatability
O4	Vibes vector verified	affect-state projection
O5	receptor map aligned	gain-field projection
O6	graph dimensions checked	parcel dynamics
O7	runtime replayable	trajectory behavior
O8	disagreement traceable	discovery claims
O9	phase report passes	next phase promotion

⸻

9. Runtime/compute plan

Local baseline: RX 6900 XT 16 GB

Use:

ONNX Runtime + DirectML
small/medium frozen probes
CPU sparse graph math
Zarr/Parquet cache

The graph side is tiny; the bottleneck is probe inference. ONNX Runtime DirectML is the practical Windows/AMD path because it targets commodity GPUs with broad hardware support.  ￼

Expected runtime

Phase	Runtime per artifact	Notes
Alpha	seconds–1 min	two probes + toy graph
Beta	10 sec–5 min	real priors + modest probes
Gamma	minutes/artifact	full probe ensemble/sweeps
Large batch	overnight	cache makes replays cheap

⸻

10. Build sequence

Sprint 1 — Snap executable spine

1. parser/serializer,
2. edge families,
3. weighted edge parser,
4. dynamic operator-ref resolver,
5. phase suffix naming convention.

Done when: the graph emitted earlier roundtrips and operator refs resolve.

⸻

Sprint 2 — Alpha totality

1. artifact intake,
2. two frozen probes,
3. cache,
4. Vibes projection,
5. mock receptor gain,
6. mock graph,
7. toy walk,
8. trace report.

Done when: one artifact generates deterministic totality trace.

⸻

Sprint 3 — Beta replacements

1. receptor bridge,
2. neuromaps transform,
3. real 360 parcel graph,
4. graph-neural-field-compatible walk,
5. disagreement/width-v1,
6. beta report.

Done when: alpha mocks are replaced and all promoted/wrapped/replaced nodes are labeled.

⸻

Sprint 4 — Gamma probe apparatus

1. full probe registry,
2. latent-axis sweeps,
3. sensitivity evaluator,
4. prior ensemble,
5. cross-projection readout.

Done when: system shows where frozen models and public priors agree/disagree.

⸻

Sprint 5 — Gamma dynamics

1. magnetic Laplacian,
2. wavelet filtering,
3. objective/narrative dual path,
4. reduction test to beta Laplacian.

Done when: gamma runtime reduces to beta when phase/wavelet extensions are disabled.

⸻

Sprint 6 — Discovery surface

1. graph viewer,
2. trace viewer,
3. disagreement viewer,
4. export to Snap,
5. phase-gate report.

Done when: every UI claim is traceable to source records.

⸻

11. Non-obvious implementation constraints

1. Do not compare alpha to gamma.
    Alpha verifies shape, not truth.
2. Do not collapse probes into one score.
    The useful surface is disagreement.
3. Do not recompute probes in graph experiments.
    Cache first, replay many times.
4. Do not use phase in payload names.
    ProbeCache-Β is lifecycle. ProbeCache❇logits is data.
5. Do not make receptor maps clinical.
    They are priors/gain fields, not measurements.
6. Do not add new Snap syntax.
    Use edge families, typed nodes, operator refs, and semantic names.

⸻

12. Final acceptance definition

Alpha accepted when

one artifact → deterministic totality trace

Beta accepted when

all alpha mocks replaced or explicitly deferred,
real priors installed,
Snap runtime replayable

Gamma accepted when

full probe/prior graph exposes agreement, disagreement,
width, energy, and traceable operator paths
without training, clinical data, or human bottlenecks

`#end resonance/implementation-plan.md`
