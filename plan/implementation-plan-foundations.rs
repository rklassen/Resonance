Implementation plan foundations

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

Reasoning: Rust owns deterministic graph execution and Snap serialization.
Python owns neuroscience tooling because neuromaps, atlas handling, and
model-conversion ecosystem are Python-first. Zarr is used for n-dimensional
arrays because it supports chunking and compression.

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
  artifact_form: Image | Text | Snap | Source,
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
  output_form: Embedding | Logits | Labels,
  vector_ref: String,
  created_at: String,
}

Verified when: same artifact/model/prompt gives same output within tolerance.

Use ONNX Runtime + DirectML on Windows because DirectML accelerates ONNX models
on commodity GPUs without vendor-specific extensions.

⸻

3.3 Vibes state record

VibesState {
  state_11d: [f32; 11],
  state_12d: [f32; 12],
  collapse_flags: Vec<String>,
  scale_roles: Vec<String>,
  source_probe_ids: Vec<String>,
}

⸻

3.4 Receptor bridge record

ReceptorGain {
  parcel_id: String,
  receptor_family: String,
  gain: f32,
  source_map: String,
  transform_id: String,
}

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

`#end resonance/implementation-plan-foundations.rs`