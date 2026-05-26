use crate::{OperatorExecutionRecord, TraceRecord, TraceStep};

use super::artifact::AlphaArtifact;
use super::graph::AlphaParcelGraph;
use super::probe::AlphaProbeRun;
use super::receptor::AlphaGain;
use super::snap::AlphaSnapDocument;
use super::vibes::AlphaVibes;
use super::walk::AlphaWalk;

#[allow(clippy::too_many_arguments)]
pub(crate) fn totality_complete(
    artifact: &AlphaArtifact,
    embedding: &AlphaProbeRun,
    labels: &AlphaProbeRun,
    vibes: &AlphaVibes,
    gain: &AlphaGain,
    graph: &AlphaParcelGraph,
    walk: &AlphaWalk,
    report_execution: &OperatorExecutionRecord,
    trace: &TraceRecord,
    steps: &[TraceStep],
    snap: &AlphaSnapDocument,
) -> bool {
    const REQUIRED_STEPS: [&str; 9] = [
        "trace-step-intake",
        "trace-step-embedding-probe",
        "trace-step-embedding-cache",
        "trace-step-label-probe",
        "trace-step-label-cache",
        "trace-step-vibes",
        "trace-step-gain",
        "trace-step-walk",
        "trace-step-report",
    ];
    const REQUIRED_OBSERVATIONS: [(&str, &str); 4] = [
        ("o101", "Observe-Snap-Roundtrip❇alpha"),
        ("o103", "Observe-Probe-Stability❇alpha"),
        ("o105", "Observe-Vibes-Vector❇alpha"),
        ("o109", "Observe-Totality-Trace❇alpha"),
    ];
    const REQUIRED_VERIFY_EDGES: [(&str, &str); 4] =
        [("a101", "o101"), ("a103", "o103"), ("a105", "o105"), ("a109", "o109")];

    let expected_operators = [
        artifact.execution.operator.0.as_str(),
        embedding.execution.probe.0.as_str(),
        embedding.cache_execution.operator.0.as_str(),
        labels.execution.probe.0.as_str(),
        labels.cache_execution.operator.0.as_str(),
        vibes.execution.operator.0.as_str(),
        gain.execution.operator.0.as_str(),
        walk.execution.operator.0.as_str(),
        report_execution.operator.0.as_str(),
    ];
    let expected_payloads = vec![
        embedding.payload.id.clone(),
        labels.payload.id.clone(),
        vibes.payload_12d.id.clone(),
        vibes.payload_11d.id.clone(),
        gain.payload.id.clone(),
        walk.payload.id.clone(),
    ];
    let expected_executions = vec![
        artifact.execution.id.clone(),
        embedding.execution.id.clone(),
        embedding.cache_execution.id.clone(),
        labels.execution.id.clone(),
        labels.cache_execution.id.clone(),
        vibes.execution.id.clone(),
        gain.execution.id.clone(),
        walk.execution.id.clone(),
        report_execution.id.clone(),
    ];

    let observations_present = REQUIRED_OBSERVATIONS
        .iter()
        .all(|(id, name)| snap.nodes.iter().any(|node| node.id == *id && node.name == *name));
    let verify_edges_present = snap
        .edge_groups
        .iter()
        .find(|group| group.family == "verify")
        .map(|group| {
            REQUIRED_VERIFY_EDGES
                .iter()
                .all(|(from, to)| group.edges.iter().any(|edge| edge.0 == *from && edge.1 == *to))
        })
        .unwrap_or(false);
    let registers_present = snap.registers.iter().any(|register| {
        register.key == "artifact_hash" && register.value == artifact.record.hash.digest_hex
    }) && snap
        .registers
        .iter()
        .any(|register| register.key == "trace_id" && register.value == trace.id.0)
        && snap.registers.iter().any(|register| {
            register.key == "graph_nodes" && register.value == graph.node_count.to_string()
        });

    trace.source_artifacts == vec![artifact.record.id.clone()]
        && trace.operator_executions == expected_executions
        && trace.payloads == expected_payloads
        && trace.path.0 == format!("snap://alpha/trace/{}", artifact.record.id.0)
        && steps.len() == REQUIRED_STEPS.len()
        && steps.iter().map(|step| step.id.0.as_str()).eq(REQUIRED_STEPS)
        && steps.iter().map(|step| step.operator.0.as_str()).eq(expected_operators)
        && observations_present
        && verify_edges_present
        && registers_present
}
