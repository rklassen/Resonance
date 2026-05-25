use std::sync::OnceLock;

use serde::Deserialize;

use super::BetaError;

const PUBLIC_FIXTURE_JSON: &str = include_str!("../../artifacts/beta/public-fixtures-v1.json");

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BetaPublicFixtures {
    pub generated: i64,
    pub atlas: BetaAtlasFixture,
    pub parcels: Vec<BetaFixtureParcel>,
    pub priors: Vec<BetaFixturePrior>,
    pub graph: BetaFixtureGraph,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BetaAtlasFixture {
    pub id: String,
    pub xml_url: String,
    pub xml_sha256: String,
    pub volume_url: String,
    pub volume_sha256: String,
    pub transform: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BetaFixtureParcel {
    pub node: usize,
    pub id: usize,
    pub name: String,
    pub hemi: String,
    pub base: String,
    pub homologue_name: String,
    pub centroid: [f32; 3],
    pub voxel_count: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BetaFixturePrior {
    pub id: String,
    pub source: String,
    pub desc: String,
    pub space: String,
    pub res: String,
    pub url: String,
    pub sha256: String,
    pub transform: String,
    pub raw_min: f32,
    pub raw_max: f32,
    pub coverage_min: f32,
    pub coverage_max: f32,
    pub values: Vec<f32>,
    pub value_hash: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BetaFixtureGraph {
    pub schema: String,
    pub edge_count: usize,
    pub edges: Vec<BetaFixtureEdge>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct BetaFixtureEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
}

pub fn load_public_fixtures() -> Result<&'static BetaPublicFixtures, BetaError> {
    static FIXTURES: OnceLock<Result<BetaPublicFixtures, BetaError>> = OnceLock::new();

    FIXTURES
        .get_or_init(|| {
            let fixtures = serde_json::from_str::<BetaPublicFixtures>(PUBLIC_FIXTURE_JSON)
                .map_err(|error| BetaError::new(format!("beta fixture parse failed: {error}")))?;
            validate_fixtures(&fixtures)?;
            Ok(fixtures)
        })
        .as_ref()
        .map_err(|error| error.clone())
}

fn validate_fixtures(fixtures: &BetaPublicFixtures) -> Result<(), BetaError> {
    if fixtures.parcels.len() != 360 {
        return Err(BetaError::new("beta fixtures must declare 360 parcels"));
    }
    if fixtures.priors.is_empty() {
        return Err(BetaError::new("beta fixtures must declare at least one real prior"));
    }
    if fixtures.graph.edge_count == 0 || fixtures.graph.edges.is_empty() {
        return Err(BetaError::new("beta fixtures must declare graph edges"));
    }
    if fixtures.graph.edge_count != fixtures.graph.edges.len() {
        return Err(BetaError::new("beta fixture graph edge count mismatch"));
    }

    for (expected_node, parcel) in fixtures.parcels.iter().enumerate() {
        if parcel.node != expected_node {
            return Err(BetaError::new("beta fixture parcels must use contiguous node ids"));
        }
        if !matches!(parcel.hemi.as_str(), "L" | "R") {
            return Err(BetaError::new("beta fixture parcels must declare hemisphere labels"));
        }
        if parcel.voxel_count == 0 {
            return Err(BetaError::new("beta fixture parcels must have non-zero voxel counts"));
        }
    }

    for prior in &fixtures.priors {
        if prior.values.len() != fixtures.parcels.len() {
            return Err(BetaError::new("beta prior width must match parcel count"));
        }
        if !(0.0..=1.0).contains(&prior.coverage_min) || !(0.0..=1.0).contains(&prior.coverage_max)
        {
            return Err(BetaError::new("beta prior coverage must be normalized"));
        }
        if prior.values.iter().any(|value| !value.is_finite()) {
            return Err(BetaError::new("beta priors must be finite"));
        }
    }

    for edge in &fixtures.graph.edges {
        if edge.from >= fixtures.parcels.len() || edge.to >= fixtures.parcels.len() {
            return Err(BetaError::new("beta graph edges cannot reference dangling parcel nodes"));
        }
        if edge.from == edge.to {
            return Err(BetaError::new("beta graph edges cannot self-loop"));
        }
        if !(edge.weight.is_finite() && edge.weight > 0.0) {
            return Err(BetaError::new("beta graph edges must be positive and finite"));
        }
    }

    Ok(())
}
