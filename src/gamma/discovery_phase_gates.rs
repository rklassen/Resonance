use crate::{
    alpha::CacheStatus,
    beta::BetaRun,
    ClaimRecord, GateDecision, GateResult,
};

use super::{
    GammaCrossProjectionReadout, GammaDualPathRuntime, GammaPriorAlignment,
    GammaPriorEnsembleSuite, GammaPriorSource, GammaProbeSuite,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaPhaseGateRow {
    pub gate: String,
    pub inspect_after: String,
    pub allowed_inspection: String,
    pub decision: GateDecision,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaPhaseGateReport {
    pub rows: Vec<GammaPhaseGateRow>,
    pub promotion_decision: GateDecision,
    pub blocking_reasons: Vec<String>,
    pub next_observable: String,
}

pub struct GammaPhaseGateInputs<'a> {
    pub beta: &'a BetaRun,
    pub probe_suite: &'a GammaProbeSuite,
    pub prior_ensemble: &'a GammaPriorEnsembleSuite,
    pub dual_path: &'a GammaDualPathRuntime,
    pub readout: &'a GammaCrossProjectionReadout,
    pub discovery_gate_result: &'a GateResult,
    pub discovery_claim: &'a ClaimRecord,
    pub discovery_snap_text: &'a str,
}

pub fn build_gamma_phase_gate_report(inputs: GammaPhaseGateInputs<'_>) -> GammaPhaseGateReport {
    let o1_ready = !inputs.beta.report.snap_text.is_empty() && !inputs.discovery_snap_text.is_empty();
    let o2_ready = !inputs.beta.artifact.record.hash.digest_hex.is_empty();
    let o3_ready = !inputs.probe_suite.families.is_empty()
        && inputs.probe_suite.families.iter().all(|family| !family.run.cache_key.as_string().is_empty())
        && matches!(inputs.beta.embedding_probe.cache_status, CacheStatus::Hit | CacheStatus::Miss)
        && matches!(inputs.beta.label_probe.cache_status, CacheStatus::Hit | CacheStatus::Miss);
    let o4_ready = inputs.beta.vibes.validation.ranges_valid
        && inputs.beta.vibes.validation.collapse_valid
        && inputs.beta.vibes.validation.roles_valid;
    let aligned_receptor_priors = inputs
        .prior_ensemble
        .priors
        .iter()
        .filter(|prior| prior.source == GammaPriorSource::ReceptorMaps)
        .collect::<Vec<_>>();
    let o5_ready = !aligned_receptor_priors.is_empty()
        && aligned_receptor_priors.iter().all(|prior| {
            prior.alignment == GammaPriorAlignment::CoordinateAligned
                && prior.atlas_id.is_some()
                && prior.transform_id.is_some()
        });
    let o6_ready = inputs.dual_path.narrative_path.parcel_feedback.gate_result.decision == GateDecision::Pass
        && inputs.dual_path.objective_path.runtime.state_after.len() == inputs.beta.graph.node_count
        && inputs.beta.walk.state_after.len() == inputs.beta.graph.node_count;
    let o7_ready = inputs.beta.report.gate_result.decision == GateDecision::Pass
        && inputs.dual_path.objective_path.runtime.gate_result.decision == GateDecision::Pass;
    let o8_ready = inputs.readout.gate_result.decision == GateDecision::Pass
        && inputs
            .readout
            .pairs
            .iter()
            .all(|pair| pair.disagreement <= f32::EPSILON || pair.localizer.is_some());
    let o9_ready = inputs.discovery_gate_result.decision == GateDecision::Pass
        && inputs.discovery_claim.phase_scope.is_some()
        && [o1_ready, o2_ready, o3_ready, o4_ready, o5_ready, o6_ready, o7_ready, o8_ready]
            .into_iter()
            .all(|ready| ready);

    let mut blocking_reasons = Vec::new();
    if inputs
        .prior_ensemble
        .priors
        .iter()
        .any(|prior| prior.source != GammaPriorSource::ReceptorMaps && prior.alignment != GammaPriorAlignment::CoordinateAligned)
    {
        blocking_reasons.push(
            "non-receptor public priors remain blocked until declared sources and transforms exist"
                .into(),
        );
    }
    blocking_reasons.push(
        "interactive discovery viewer stack is still incomplete beyond the typed report/export surfaces"
            .into(),
    );

    let rows = vec![
        phase_gate_row(
            "O1",
            "Snap roundtrip",
            "syntax/canonical form",
            o1_ready,
            format!(
                "beta snap report and gamma discovery snap export are both populated for {}",
                inputs.beta.artifact.record.id.0,
            ),
        ),
        phase_gate_row(
            "O2",
            "artifact hash",
            "intake determinism",
            o2_ready,
            format!("artifact hash is {}", inputs.beta.artifact.record.hash.digest_hex),
        ),
        phase_gate_row(
            "O3",
            "probe cache",
            "probe repeatability",
            o3_ready,
            format!(
                "embedding={}, labels={}, gamma probe families={}",
                cache_status_name(&inputs.beta.embedding_probe.cache_status),
                cache_status_name(&inputs.beta.label_probe.cache_status),
                inputs.probe_suite.families.len(),
            ),
        ),
        phase_gate_row(
            "O4",
            "Vibes vector verified",
            "affect-state projection",
            o4_ready,
            "signed 12d Vibes vector passed range, collapse, and role validation".into(),
        ),
        phase_gate_row(
            "O5",
            "receptor map aligned",
            "gain-field projection",
            o5_ready,
            format!(
                "aligned receptor priors={} blocked non-receptor priors={}",
                aligned_receptor_priors.len(),
                inputs.prior_ensemble.priors.len().saturating_sub(aligned_receptor_priors.len()),
            ),
        ),
        phase_gate_row(
            "O6",
            "graph dimensions checked",
            "parcel dynamics",
            o6_ready,
            format!("graph node_count={} objective/narrative widths match", inputs.beta.graph.node_count),
        ),
        phase_gate_row(
            "O7",
            "runtime replayable",
            "trajectory behavior",
            o7_ready,
            "beta privileged path and gamma objective reduction gates both pass".into(),
        ),
        phase_gate_row(
            "O8",
            "disagreement traceable",
            "discovery claims",
            o8_ready,
            format!("cross-projection pairs={} and every nonzero disagreement is localized", inputs.readout.pairs.len()),
        ),
        GammaPhaseGateRow {
            gate: "O9".into(),
            inspect_after: "phase report passes".into(),
            allowed_inspection: "next phase promotion".into(),
            decision: if o9_ready { GateDecision::Pass } else { GateDecision::Fail },
            detail: if o9_ready {
                "phase-local discovery report is complete enough to support a promotion decision".into()
            } else {
                "phase-local discovery report is missing one or more prerequisite gate conditions".into()
            },
        },
    ];

    GammaPhaseGateReport {
        rows,
        promotion_decision: GateDecision::Blocked,
        blocking_reasons,
        next_observable: "declared non-receptor public prior source or richer interactive discovery viewer".into(),
    }
}

fn phase_gate_row(
    gate: &str,
    inspect_after: &str,
    allowed_inspection: &str,
    ready: bool,
    detail: String,
) -> GammaPhaseGateRow {
    GammaPhaseGateRow {
        gate: gate.into(),
        inspect_after: inspect_after.into(),
        allowed_inspection: allowed_inspection.into(),
        decision: if ready { GateDecision::Pass } else { GateDecision::Fail },
        detail,
    }
}

fn cache_status_name(status: &CacheStatus) -> &'static str {
    match status {
        CacheStatus::Hit => "Hit",
        CacheStatus::Miss => "Miss",
    }
}