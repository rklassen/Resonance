use crate::{
    alpha::VIBES_12D,
    beta::BetaRun, ClaimRecord, GateResult, TraceRecord,
};

use super::{
    GammaCrossProjectionReadout, GammaDisagreementLocalizer, GammaDualPathRuntime,
    GammaProbeSuite,
};

#[derive(Clone, Debug, PartialEq)]
pub struct GammaDiscoveryViews {
    pub dag_trace: Vec<GammaDagTraceRow>,
    pub probe_matrix: Vec<GammaProbeMatrixRow>,
    pub vibes_vector: Vec<GammaVibesAxisRow>,
    pub receptor_table: Vec<GammaReceptorTableRow>,
    pub parcel_trajectory: Vec<GammaParcelTrajectoryRow>,
    pub disagreement_graph: Vec<GammaDisagreementEdge>,
    pub width_energy: GammaWidthEnergyReadout,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GammaDagTraceRow {
    pub label: String,
    pub trace_id: String,
    pub path: String,
    pub gate_results: Vec<String>,
    pub claims: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaProbeMatrixRow {
    pub family: String,
    pub model_id: String,
    pub output_contract: String,
    pub payload_id: String,
    pub signal_mean: f32,
    pub sample_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaVibesAxisRow {
    pub axis: String,
    pub role: String,
    pub value: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaReceptorTableRow {
    pub prior_id: String,
    pub family: String,
    pub target: String,
    pub coefficient: f32,
    pub evidence_axes: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaParcelTrajectoryRow {
    pub parcel_id: usize,
    pub parcel_name: String,
    pub beta_state: f32,
    pub gamma_state: f32,
    pub narrative_feedback: f32,
    pub gamma_delta: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaDisagreementEdge {
    pub label: String,
    pub disagreement: f32,
    pub localizer: Option<String>,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GammaWidthEnergyReadout {
    pub probe_disagreement: f32,
    pub receptor_projection_disagreement: f32,
    pub graph_spread: f32,
    pub energy_proxy: f32,
    pub unsupported_edges: usize,
}

pub fn build_gamma_discovery_views(
    beta: &BetaRun,
    probe_suite: &GammaProbeSuite,
    dual_path: &GammaDualPathRuntime,
    readout: &GammaCrossProjectionReadout,
    trace: &TraceRecord,
    gate_result: &GateResult,
    claim: &ClaimRecord,
) -> GammaDiscoveryViews {
    GammaDiscoveryViews {
        dag_trace: vec![
            GammaDagTraceRow {
                label: "beta-privileged-path".into(),
                trace_id: beta.report.trace.id.0.clone(),
                path: beta.report.trace.path.0.clone(),
                gate_results: beta
                    .report
                    .trace
                    .gate_results
                    .iter()
                    .map(|gate_id| gate_id.0.clone())
                    .collect(),
                claims: beta
                    .report
                    .trace
                    .claims
                    .iter()
                    .map(|claim_id| claim_id.0.clone())
                    .collect(),
            },
            GammaDagTraceRow {
                label: "gamma-objective-path".into(),
                trace_id: dual_path.objective_path.trace.id.0.clone(),
                path: dual_path.objective_path.trace.path.0.clone(),
                gate_results: dual_path
                    .objective_path
                    .trace
                    .gate_results
                    .iter()
                    .map(|gate_id| gate_id.0.clone())
                    .collect(),
                claims: dual_path
                    .objective_path
                    .trace
                    .claims
                    .iter()
                    .map(|claim_id| claim_id.0.clone())
                    .collect(),
            },
            GammaDagTraceRow {
                label: "gamma-narrative-path".into(),
                trace_id: dual_path.narrative_path.trace.id.0.clone(),
                path: dual_path.narrative_path.trace.path.0.clone(),
                gate_results: dual_path
                    .narrative_path
                    .trace
                    .gate_results
                    .iter()
                    .map(|gate_id| gate_id.0.clone())
                    .collect(),
                claims: dual_path
                    .narrative_path
                    .trace
                    .claims
                    .iter()
                    .map(|claim_id| claim_id.0.clone())
                    .collect(),
            },
            GammaDagTraceRow {
                label: "gamma-cross-projection-readout".into(),
                trace_id: readout.trace.id.0.clone(),
                path: readout.trace.path.0.clone(),
                gate_results: readout
                    .trace
                    .gate_results
                    .iter()
                    .map(|gate_id| gate_id.0.clone())
                    .collect(),
                claims: readout
                    .trace
                    .claims
                    .iter()
                    .map(|claim_id| claim_id.0.clone())
                    .collect(),
            },
            GammaDagTraceRow {
                label: "gamma-discovery-surface".into(),
                trace_id: trace.id.0.clone(),
                path: trace.path.0.clone(),
                gate_results: vec![gate_result.gate_result_id.0.clone()],
                claims: vec![claim.id.0.clone()],
            },
        ],
        probe_matrix: probe_suite
            .families
            .iter()
            .map(|family| GammaProbeMatrixRow {
                family: family.family.name.clone(),
                model_id: family.family.model_id.clone(),
                output_contract: family.family.output_contract.clone(),
                payload_id: family.run.payload.id.0.clone(),
                signal_mean: family.run.values.iter().sum::<f32>() / family.run.values.len() as f32,
                sample_count: family.run.values.len(),
            })
            .collect(),
        vibes_vector: VIBES_12D
            .iter()
            .zip(beta.vibes.signed_12d.iter())
            .map(|(axis, value)| GammaVibesAxisRow {
                axis: axis.name.into(),
                role: axis.role.into(),
                value: *value,
            })
            .collect(),
        receptor_table: beta
            .gain
            .terms
            .iter()
            .map(|term| GammaReceptorTableRow {
                prior_id: term.prior_id.clone(),
                family: term.family.clone(),
                target: term.target.clone(),
                coefficient: term.coefficient,
                evidence_axes: term.evidence_axes.clone(),
            })
            .collect(),
        parcel_trajectory: beta
            .graph
            .parcel_ids
            .iter()
            .zip(beta.graph.parcel_names.iter())
            .zip(beta.walk.state_after.iter())
            .zip(dual_path.objective_path.runtime.state_after.iter())
            .zip(dual_path.narrative_path.parcel_feedback.vector.iter())
            .map(|((((parcel_id, parcel_name), beta_state), gamma_state), narrative_feedback)| {
                GammaParcelTrajectoryRow {
                    parcel_id: *parcel_id,
                    parcel_name: parcel_name.clone(),
                    beta_state: *beta_state,
                    gamma_state: *gamma_state,
                    narrative_feedback: *narrative_feedback,
                    gamma_delta: gamma_state - beta_state,
                }
            })
            .collect(),
        disagreement_graph: readout
            .pairs
            .iter()
            .map(|pair| GammaDisagreementEdge {
                label: pair.name.clone(),
                disagreement: pair.disagreement,
                localizer: render_localizer(pair.localizer.as_ref()),
                detail: pair.detail.clone(),
            })
            .collect(),
        width_energy: GammaWidthEnergyReadout {
            probe_disagreement: beta.disagreement.probe_disagreement,
            receptor_projection_disagreement: beta.disagreement.receptor_projection_disagreement,
            graph_spread: beta.disagreement.graph_spread,
            energy_proxy: beta.disagreement.energy_proxy,
            unsupported_edges: beta.disagreement.unsupported_edges,
        },
    }
}

fn render_localizer(localizer: Option<&GammaDisagreementLocalizer>) -> Option<String> {
    match localizer {
        Some(GammaDisagreementLocalizer::Probe(value)) => Some(format!("probe:{value}")),
        Some(GammaDisagreementLocalizer::Prompt(value)) => Some(format!("prompt:{value}")),
        Some(GammaDisagreementLocalizer::Prior(value)) => Some(format!("prior:{value}")),
        Some(GammaDisagreementLocalizer::Transform(value)) => Some(format!("transform:{value}")),
        Some(GammaDisagreementLocalizer::Operator(value)) => Some(format!("operator:{value}")),
        Some(GammaDisagreementLocalizer::GraphEdge(value)) => Some(format!("graph-edge:{value}")),
        None => None,
    }
}