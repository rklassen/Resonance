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
Verified by the gamma probe-validity evaluator and the unstable-axis promotion guard test.

✅ G4 — Prior-Ensemble-Γ
Verified by the gamma prior-ensemble observable and the aligned-or-blocked prior test.

✅ G5 — Receptor-Bridge-Γ
Verified by the gamma receptor-bridge ensemble observable and the provenanced family comparison test.

✅ G6 — Dual-Path-Runtime-Γ
Verified by `run_gamma_dual_path_runtime()` and the independent path-trace test.

✅ G7 — Magnetic-Wavelet-Runtime-Γ
Verified by the directed-phase, wavelet, and recirculation runtime implementation plus the live-path and beta-reduction gamma phase tests.

⚠️ G8 — Cross-Projection-Readout-Γ
Planned, but cross-projection disagreement localization is not implemented yet.

⚠️ G9 — Discovery-Surface-Γ
Planned, but the gamma viewer/export surface and visible-claim trace links are not implemented yet.

Current execution order

1. Read `implementation-plan-foundations.rs` for shared contracts and Snap work.
2. Read `implementation-plan-alpha-beta.rs` for promoted substrate behavior.
3. Read `implementation-plan-gamma.rs` for the active gamma milestone chain.

Current active milestone

The next work package is `G8 — Cross-Projection-Readout-Γ` in
`implementation-plan-gamma.rs`.

`#end resonance/implementation-plan.md`
