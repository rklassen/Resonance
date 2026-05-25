Implementation plan gamma

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

Split one gamma runtime into traced paths:

objective path:
  perceptual/artifact features → parcel graph substrate
narrative path:
   semantic/affect-led trace → shared Vibes/receptor-gain substrate

Verify: each path independently produces trace before comparison or runtime replacement.

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

`#end resonance/implementation-plan-gamma.rs`