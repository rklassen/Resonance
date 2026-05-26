use crate::{
    beta::BetaRun, ClaimRecord, ClaimStatus, GateDecision, GateResult,
    GammaPriorAlignment, ValueRef,
};

use super::{
    GammaDiscoveryViews, GammaPhaseGateReport, GammaPriorEnsembleSuite,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaDiscoveryReport {
    pub export: ValueRef,
    pub text: String,
}

pub fn build_gamma_discovery_report(
    beta: &BetaRun,
    prior_ensemble: &GammaPriorEnsembleSuite,
    views: &GammaDiscoveryViews,
    phase_gate_report: &GammaPhaseGateReport,
    claim: &ClaimRecord,
    gate_result: &GateResult,
) -> GammaDiscoveryReport {
    let artifact_id = beta.artifact.record.id.0.as_str();
    let report_export = gamma_discovery_report_export(artifact_id);
    let snap_export = gamma_discovery_snap_export(artifact_id);
    let mut lines = vec![
        "# Gamma Discovery Surface Report".into(),
        String::new(),
        format!("artifact_id: {artifact_id}"),
        format!("artifact_hash: {}", beta.artifact.record.hash.digest_hex),
        format!(
            "phase: {}",
            claim.phase_scope.as_ref().map_or("-", |phase| phase.0.as_str()),
        ),
        format!("report_export: {}", report_export.0),
        format!("snap_export: {snap_export}"),
        String::new(),
        "## Claim".into(),
        format!("claim_id: {}", claim.id.0),
        format!("status: {}", claim_status_label(&claim.status)),
        format!("statement: {}", claim.statement),
        format!(
            "support_traces: {}",
            comma_list(claim.support_traces.iter().map(|trace_id| trace_id.0.clone())),
        ),
        format!(
            "support_gates: {}",
            comma_list(
                claim
                    .support_gate_results
                    .iter()
                    .map(|gate_id| gate_id.0.clone()),
            ),
        ),
        format!(
            "support_snaps: {}",
            comma_list(claim.support_snaps.iter().map(|snap| snap.0.clone())),
        ),
        String::new(),
        "## DAG Trace".into(),
        String::new(),
        "## Probe Matrix".into(),
    ];

    for row in &views.dag_trace {
        lines.push(format!(
            "- {}: {} ({})",
            row.label, row.trace_id, row.path,
        ));
        lines.push(format!("  gates: {}", comma_list(row.gate_results.iter().cloned())));
        lines.push(format!("  claims: {}", comma_list(row.claims.iter().cloned())));
    }

    for row in &views.probe_matrix {
        lines.push(format!(
            "- {} {} {:.6} n={}",
            row.family, row.model_id, row.signal_mean, row.sample_count,
        ));
        lines.push(format!("  payload: {}", row.payload_id));
        lines.push(format!("  contract: {}", row.output_contract));
    }

    lines.extend([
        String::new(),
        "## Vibes Vector".into(),
    ]);

    for row in &views.vibes_vector {
        lines.push(format!("- {} [{}]: {:+.6}", row.axis, row.role, row.value));
    }

    lines.extend([
        String::new(),
        "## Receptor Table".into(),
    ]);

    for row in &views.receptor_table {
        lines.push(format!(
            "- {} {} {:+.6} ({})",
            row.family,
            row.target,
            row.coefficient,
            row.prior_id,
        ));
        lines.push(format!("  evidence_axes: {}", comma_list(row.evidence_axes.iter().cloned())));
    }

    lines.extend([
        String::new(),
        "## Parcel Trajectory".into(),
        format!("trajectory_rows: {}", views.parcel_trajectory.len()),
    ]);

    for row in views.parcel_trajectory.iter().take(12) {
        lines.push(format!(
            "- {} {} beta={:+.6} gamma={:+.6} narrative={:+.6} delta={:+.6}",
            row.parcel_id,
            row.parcel_name,
            row.beta_state,
            row.gamma_state,
            row.narrative_feedback,
            row.gamma_delta,
        ));
    }

    lines.extend([
        String::new(),
        "## Disagreement Graph".into(),
    ]);

    for edge in &views.disagreement_graph {
        lines.push(format!(
            "- {}: {:+.6} [{}]",
            edge.label,
            edge.disagreement,
            edge.localizer.as_deref().unwrap_or("none"),
        ));
        lines.push(format!("  detail: {}", edge.detail));
    }

    lines.extend([
        String::new(),
        "## Phase Gate Report".into(),
    ]);

    for row in &phase_gate_report.rows {
        lines.push(format!(
            "- {} {} {} {}",
            row.gate,
            row.inspect_after,
            row.allowed_inspection,
            render_gate_decision(&row.decision),
        ));
        lines.push(format!("  detail: {}", row.detail));
    }
    lines.push(format!(
        "promotion_decision: {}",
        render_gate_decision(&phase_gate_report.promotion_decision),
    ));
    lines.push(format!(
        "blocking_reasons: {}",
        comma_list(phase_gate_report.blocking_reasons.iter().cloned()),
    ));
    lines.push(format!("next_observable: {}", phase_gate_report.next_observable));

    lines.extend([
        String::new(),
        "## Width Energy".into(),
        format!(
            "probe_disagreement: {:+.6}",
            views.width_energy.probe_disagreement,
        ),
        format!(
            "receptor_projection_disagreement: {:+.6}",
            views.width_energy.receptor_projection_disagreement,
        ),
        format!("graph_spread: {:+.6}", views.width_energy.graph_spread),
        format!("energy_proxy: {:+.6}", views.width_energy.energy_proxy),
        format!("unsupported_edges: {}", views.width_energy.unsupported_edges),
        String::new(),
        "## Gate States".into(),
        format!(
            "- beta privileged path: {}",
            render_gate_decision(&beta.report.gate_result.decision),
        ),
        format!(
            "- gamma discovery surface: {} ({})",
            render_gate_decision(&gate_result.decision),
            gate_result.gate_result_id.0,
        ),
        String::new(),
        "## Priors".into(),
    ]);

    for prior in &prior_ensemble.priors {
        lines.push(format!(
            "- {} [{}]",
            prior.prior_id,
            render_prior_alignment(&prior.alignment),
        ));
        lines.push(format!(
            "  source_record: {}",
            prior.source_record.as_deref().unwrap_or("-"),
        ));
        lines.push(format!(
            "  atlas_id: {}",
            prior.atlas_id.as_deref().unwrap_or("-"),
        ));
        lines.push(format!(
            "  transform_id: {}",
            prior.transform_id.as_deref().unwrap_or("-"),
        ));
        lines.push(format!("  detail: {}", prior.detail));
        lines.push(format!(
            "  required_follow_up: {}",
            prior.required_follow_up.as_deref().unwrap_or("-"),
        ));
    }

    GammaDiscoveryReport {
        export: report_export,
        text: lines.join("\n"),
    }
}

pub fn gamma_discovery_snap(
    artifact_id: &str,
    artifact_hash: &str,
    probe_families: usize,
    priors: usize,
    cross_projection_pairs: usize,
    receptor_families: usize,
) -> String {
    [
        "🪢snap resonance-gamma-discovery".into(),
        ".graph {".into(),
        format!(" id: gamma-discovery-{artifact_id},"),
        " name: 'resonance-gamma-discovery',".into(),
        " version: 0.8,".into(),
        "}".into(),
        "nodes {".into(),
        " object { id: g101, name: 'Snap-Spine-Γ', type: SnapSpine, }".into(),
        " object { id: g102, name: 'Probe-Suite-Γ', type: ProbeSuite, }".into(),
        " object { id: g103, name: 'Latent-Axis-Sweep-Γ', type: LatentAxisSweep, }".into(),
        " object { id: g104, name: 'Probe-Validity-Evaluator-Γ', type: ProbeValidity, }".into(),
        " object { id: g105, name: 'Prior-Ensemble-Γ', type: PriorEnsemble, }".into(),
        " object { id: g106, name: 'Receptor-Bridge-Γ', type: ReceptorBridge, }".into(),
        " object { id: g107, name: 'Dual-Path-Runtime-Γ', type: DualPathRuntime, }".into(),
        " object { id: g108, name: 'Magnetic-Wavelet-Runtime-Γ', type: MagneticWaveletRuntime, }".into(),
        " object { id: g109, name: 'Cross-Projection-Readout-Γ', type: CrossProjectionReadout, }".into(),
        " object { id: g110, name: 'Discovery-Surface-Γ', type: DiscoverySurface, }".into(),
        " object { id: o110, name: 'Observe-Discovery-Surface❇gamma', type: ObservationNode, }".into(),
        "}".into(),
        "edges {".into(),
        " extend { @b109 -> @g101, }".into(),
        " flow { @g101 -> @g102, @g102 -> @g103, @g103 -> @g104, @g104 -> @g105, @g105 -> @g106, @g106 -> @g107, @g107 -> @g108, @g108 -> @g109, @g109 -> @g110, }".into(),
        " verify { @g110 -> @o110, }".into(),
        "}".into(),
        "registers {".into(),
        format!(" artifact_id: '{artifact_id}',"),
        format!(" artifact_hash: '{artifact_hash}',"),
        format!(" probe_families: '{probe_families}',"),
        format!(" priors: '{priors}',"),
        format!(" receptor_families: '{receptor_families}',"),
        format!(" cross_projection_pairs: '{cross_projection_pairs}',"),
        "}".into(),
    ]
    .join("\n")
}

fn gamma_discovery_report_export(artifact_id: &str) -> ValueRef {
    ValueRef(format!(
        "file://output/reports/gamma-discovery-{artifact_id}.md",
    ))
}

fn gamma_discovery_snap_export(artifact_id: &str) -> String {
    format!("file://output/reports/gamma-discovery-{artifact_id}.snap")
}

fn claim_status_label(status: &ClaimStatus) -> &'static str {
    match status {
        ClaimStatus::ObservedFact => "ObservedFact",
        ClaimStatus::DerivedClaim => "DerivedClaim",
        ClaimStatus::BlockedClaim => "BlockedClaim",
        ClaimStatus::DeferredClaim => "DeferredClaim",
    }
}

fn render_gate_decision(decision: &GateDecision) -> &'static str {
    match decision {
        GateDecision::Pass => "Pass",
        GateDecision::Fail => "Fail",
        GateDecision::Blocked => "Blocked",
        GateDecision::Deferred => "Deferred",
    }
}

fn render_prior_alignment(alignment: &GammaPriorAlignment) -> &'static str {
    match alignment {
        GammaPriorAlignment::CoordinateAligned => "CoordinateAligned",
        GammaPriorAlignment::Blocked => "Blocked",
    }
}

fn comma_list(values: impl IntoIterator<Item = String>) -> String {
    let values = values.into_iter().collect::<Vec<_>>();
    if values.is_empty() {
        "-".into()
    } else {
        values.join(", ")
    }
}