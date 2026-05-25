use super::AlphaError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaSnapDocument {
    pub name: String,
    pub graph_id: String,
    pub version: String,
    pub types: Vec<AlphaSnapType>,
    pub nodes: Vec<AlphaSnapNode>,
    pub edge_groups: Vec<AlphaSnapEdgeGroup>,
    pub registers: Vec<AlphaSnapRegister>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaSnapType {
    pub alias: String,
    pub target: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaSnapNode {
    pub kind: String,
    pub id: String,
    pub name: String,
    pub node_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaSnapEdgeGroup {
    pub family: String,
    pub edges: Vec<(String, String)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AlphaSnapRegister {
    pub key: String,
    pub value: String,
}

impl AlphaSnapDocument {
    pub fn to_text(&self) -> String {
        let mut output = vec![format!("🪢snap {}", self.name), ".graph {".into()];
        output.push(format!(" id: {},", self.graph_id));
        output.push(format!(" name: '{}',", self.name));
        output.push(format!(" version: {},", self.version));
        output.push("}".into());

        output.push("types {".into());
        for snap_type in &self.types {
            output.push(format!(" {} = '{}',", snap_type.alias, snap_type.target));
        }
        output.push("}".into());

        output.push("nodes {".into());
        for node in &self.nodes {
            output.push(format!(" {} {{", node.kind));
            output.push(format!("  id: {},", node.id));
            output.push(format!("  name: '{}',", node.name));
            output.push(format!("  type: {},", node.node_type));
            output.push(" }".into());
        }
        output.push("}".into());

        output.push("edges {".into());
        for group in &self.edge_groups {
            output.push(format!(" {} {{", group.family));
            for (from, to) in &group.edges {
                output.push(format!("  @{} -> @{},", from, to));
            }
            output.push(" }".into());
        }
        output.push("}".into());

        output.push("registers {".into());
        for register in &self.registers {
            output.push(format!(" {}: '{}',", register.key, register.value));
        }
        output.push("}".into());
        output.join("\n")
    }

    pub fn parse(input: &str) -> Result<Self, AlphaError> {
        let lines =
            input.lines().map(str::trim).filter(|line| !line.is_empty()).collect::<Vec<_>>();
        if lines.first().is_none_or(|line| !line.starts_with("🪢snap ")) {
            return Err(AlphaError::new("alpha snap header missing"));
        }

        let name = lines[0].trim_start_matches("🪢snap ").to_string();
        let graph_id = parse_scalar(&lines, "id:")?;
        let version = parse_scalar(&lines, "version:")?;
        let types = parse_types(&lines);
        let nodes = parse_nodes(&lines)?;
        let edge_groups = parse_edges(&lines)?;
        let registers = parse_registers(&lines);

        Ok(Self {
            name: strip_quotes(&name),
            graph_id,
            version,
            types,
            nodes,
            edge_groups,
            registers,
        })
    }
}

pub fn alpha_trace_document(
    trace_id: &str,
    artifact_hash: &str,
    graph_nodes: usize,
) -> AlphaSnapDocument {
    AlphaSnapDocument {
        name: "resonance-alpha-trace".into(),
        graph_id: trace_id.into(),
        version: "0.8".into(),
        types: vec![
            AlphaSnapType {
                alias: "ArtifactRecord".into(),
                target: "artifact".into(),
            },
            AlphaSnapType {
                alias: "TraceRecord".into(),
                target: "trace".into(),
            },
        ],
        nodes: vec![
            snap_node("object", "a101", "Snap-Spine-Α", "SnapSpine"),
            snap_node("object", "a102", "Artifact-Intake-Α", "ArtifactIntake"),
            snap_node("object", "a103", "Frozen-Probe-Α", "FrozenProbe"),
            snap_node("object", "a104", "Probe-Cache-Α", "ProbeCache"),
            snap_node("object", "a105", "Vibes-Projection-Α", "VibesProjection"),
            snap_node("object", "a106", "Receptor-Gain-Α", "ReceptorGain"),
            snap_node("object", "a107", "Parcel-Graph-Α", "ParcelGraph"),
            snap_node("object", "a108", "Laplacian-Walk-Α", "LaplacianWalk"),
            snap_node("object", "a109", "Trace-Report-Α", "TraceReport"),
            snap_node("object", "o101", "Observe-Snap-Roundtrip❇alpha", "ObservationNode"),
            snap_node("object", "o103", "Observe-Probe-Stability❇alpha", "ObservationNode"),
            snap_node("object", "o105", "Observe-Vibes-Vector❇alpha", "ObservationNode"),
            snap_node("object", "o109", "Observe-Totality-Trace❇alpha", "ObservationNode"),
        ],
        edge_groups: vec![
            AlphaSnapEdgeGroup {
                family: "flow".into(),
                edges: vec![
                    ("a101".into(), "a102".into()),
                    ("a102".into(), "a103".into()),
                    ("a103".into(), "a104".into()),
                    ("a104".into(), "a105".into()),
                    ("a105".into(), "a106".into()),
                    ("a106".into(), "a107".into()),
                    ("a107".into(), "a108".into()),
                    ("a108".into(), "a109".into()),
                ],
            },
            AlphaSnapEdgeGroup {
                family: "verify".into(),
                edges: vec![
                    ("a101".into(), "o101".into()),
                    ("a103".into(), "o103".into()),
                    ("a105".into(), "o105".into()),
                    ("a109".into(), "o109".into()),
                ],
            },
        ],
        registers: vec![
            AlphaSnapRegister {
                key: "artifact_hash".into(),
                value: artifact_hash.into(),
            },
            AlphaSnapRegister {
                key: "trace_id".into(),
                value: trace_id.into(),
            },
            AlphaSnapRegister {
                key: "graph_nodes".into(),
                value: graph_nodes.to_string(),
            },
        ],
    }
}

fn parse_scalar(lines: &[&str], prefix: &str) -> Result<String, AlphaError> {
    lines
        .iter()
        .find(|line| line.starts_with(prefix))
        .map(|line| strip_quotes(line.trim_start_matches(prefix).trim_end_matches(',')))
        .ok_or_else(|| AlphaError::new(format!("alpha snap missing scalar {prefix}")))
}

fn parse_types(lines: &[&str]) -> Vec<AlphaSnapType> {
    collect_section(lines, "types")
        .into_iter()
        .filter_map(|line| line.split_once('='))
        .map(|(alias, target)| AlphaSnapType {
            alias: alias.trim().to_string(),
            target: strip_quotes(target.trim().trim_end_matches(',')),
        })
        .collect()
}

fn parse_nodes(lines: &[&str]) -> Result<Vec<AlphaSnapNode>, AlphaError> {
    let section = collect_section(lines, "nodes");
    let mut nodes = Vec::new();
    let mut index = 0;
    while index < section.len() {
        let line = section[index];
        if line.ends_with('{') {
            let kind = line.trim_end_matches('{').trim().to_string();
            let id =
                strip_quotes(section[index + 1].trim_start_matches("id:").trim_end_matches(','));
            let name =
                strip_quotes(section[index + 2].trim_start_matches("name:").trim_end_matches(','));
            let node_type =
                strip_quotes(section[index + 3].trim_start_matches("type:").trim_end_matches(','));
            nodes.push(AlphaSnapNode {
                kind,
                id,
                name,
                node_type,
            });
            index += 5;
        } else {
            index += 1;
        }
    }
    if nodes.is_empty() {
        return Err(AlphaError::new("alpha snap missing nodes"));
    }
    Ok(nodes)
}

fn parse_edges(lines: &[&str]) -> Result<Vec<AlphaSnapEdgeGroup>, AlphaError> {
    let section = collect_section(lines, "edges");
    let mut groups = Vec::new();
    let mut current_family = None::<String>;
    let mut current_edges = Vec::new();
    for line in section {
        if line.ends_with('{') {
            current_family = Some(line.trim_end_matches('{').trim().to_string());
            current_edges = Vec::new();
            continue;
        }
        if line == "}" {
            if let Some(family) = current_family.take() {
                groups.push(AlphaSnapEdgeGroup {
                    family,
                    edges: current_edges.clone(),
                });
            }
            continue;
        }
        if let Some((from, to)) =
            line.trim_end_matches(',').trim_start_matches('@').split_once(" -> @")
        {
            current_edges.push((from.to_string(), to.to_string()));
        }
    }
    if groups.is_empty() {
        return Err(AlphaError::new("alpha snap missing edges"));
    }
    Ok(groups)
}

fn parse_registers(lines: &[&str]) -> Vec<AlphaSnapRegister> {
    collect_section(lines, "registers")
        .into_iter()
        .filter_map(|line| line.split_once(':'))
        .map(|(key, value)| AlphaSnapRegister {
            key: key.trim().into(),
            value: strip_quotes(value.trim().trim_end_matches(',')),
        })
        .collect()
}

fn collect_section<'a>(lines: &'a [&'a str], name: &str) -> Vec<&'a str> {
    let mut inside = false;
    let mut depth = 0;
    let mut collected = Vec::new();
    for line in lines {
        if *line == format!("{name} {{") {
            inside = true;
            depth = 1;
            continue;
        }
        if !inside {
            continue;
        }
        if *line == "}" {
            depth -= 1;
            if depth == 0 {
                break;
            }
        } else if line.ends_with('{') {
            depth += 1;
        }
        collected.push(*line);
    }
    collected
}

fn strip_quotes(value: &str) -> String {
    value.trim().trim_matches('\'').to_string()
}

fn snap_node(kind: &str, id: &str, name: &str, node_type: &str) -> AlphaSnapNode {
    AlphaSnapNode {
        kind: kind.into(),
        id: id.into(),
        name: name.into(),
        node_type: node_type.into(),
    }
}
