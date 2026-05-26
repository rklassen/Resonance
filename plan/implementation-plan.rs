Implementation plan index

This file is the entry point for the split plan surface. The plan was divided to
keep every file under the repository hard cap without compressing or dropping
requirements.

Plan files

- `implementation-plan-foundations.rs` — repository structure, phase naming,
  core contracts, and Snap implementation tasks.
- `implementation-plan-alpha-beta.rs` — Alpha and Beta implementation plan.
- `implementation-plan-gamma.rs` — Gamma implementation, verification gates,
  runtime/compute plan, build sequence, constraints, and acceptance.

Current architecture rule

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
Verified by `run_gamma_probe_suite()` and `gamma_preserves_beta_substrate_while_extending_gamma_surfaces()`.

✅ G2 — Latent-Axis-Sweep-Γ
Verified by the fixture-backed sweep test and direct branch tests for stable-left, stable-right, and spread-gated unstable classification.

✅ G3 — Probe-Validity-Evaluator-Γ
Verified by the observational prompt-sensitivity, model-disagreement, neighborhood-instability, label-collision, and domain-mismatch checks in `run_gamma_probe_validity_suite()` plus the unstable-axis promotion gate test.

⚠️ G4 — Prior-Ensemble-Γ
Receptor-map priors are coordinate-aligned and verified by test, but functional gradients, structural connectivity, visual benchmark priors, and imagery priors remain declared blockers rather than installed sources.

✅ G5 — Receptor-Bridge-Γ
Verified by the gamma receptor-bridge ensemble observable and the provenanced family comparison test.

✅ G6 — Dual-Path-Runtime-Γ
Verified by `run_gamma_dual_path_runtime()` and the independent path-trace test.

✅ G7 — Magnetic-Wavelet-Runtime-Γ
Verified by the directed-phase, wavelet, and recirculation runtime implementation plus the live-path and beta-reduction gamma phase tests.

✅ G8 — Cross-Projection-Readout-Γ
Verified by `run_gamma_cross_projection_readout()` and the five-pair cross-projection disagreement localization test, including the traced vibes-to-receptor family comparison.

⚠️ G9 — Discovery-Surface-Γ
The traced discovery/export surface is implemented and wired through `run_gamma_discovery_surface()`, but the richer discovery viewer/export surface described in the gamma plan is not fully present yet.

Current execution order

1. Read `implementation-plan-foundations.rs` for shared contracts and Snap work.
2. Read `implementation-plan-alpha-beta.rs` for promoted substrate behavior.
3. Read `implementation-plan-gamma.rs` for the active gamma milestone chain.

Current active milestone

Gamma remains in progress. G1, G2, G3, G5, G6, G7, and G8 are implemented and verified; G4 still lacks declared non-receptor public priors, and G9 still has only the traced discovery/export surface rather than the richer viewer stack described in the gamma plan.

`#end resonance/implementation-plan.md`
