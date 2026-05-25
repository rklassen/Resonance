use std::collections::BTreeMap;

use super::{BetaError, BetaPublicFixtures};

#[derive(Clone, Debug, PartialEq)]
pub struct BetaParcelGraph {
    pub schema: String,
    pub node_count: usize,
    pub parcel_ids: Vec<usize>,
    pub parcel_names: Vec<String>,
    pub hemisphere: Vec<String>,
    pub adjacency: Vec<Vec<(usize, f32)>>,
    pub degree: Vec<f32>,
    pub non_zero_edges: usize,
}

impl BetaParcelGraph {
    pub fn from_public_fixtures(fixtures: &BetaPublicFixtures) -> Result<Self, BetaError> {
        let mut maps = vec![BTreeMap::<usize, f32>::new(); fixtures.parcels.len()];
        for edge in &fixtures.graph.edges {
            if edge.from >= fixtures.parcels.len() || edge.to >= fixtures.parcels.len() {
                return Err(BetaError::new("beta graph edge references dangling parcel node"));
            }
            maps[edge.from].insert(edge.to, edge.weight);
            maps[edge.to].insert(edge.from, edge.weight);
        }

        let adjacency = maps
            .into_iter()
            .map(|neighbors| neighbors.into_iter().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let degree = adjacency
            .iter()
            .map(|neighbors| neighbors.iter().map(|(_, weight)| weight).sum())
            .collect::<Vec<f32>>();
        let non_zero_edges = adjacency.iter().map(|neighbors| neighbors.len()).sum();
        if adjacency.iter().any(|neighbors| neighbors.is_empty()) {
            return Err(BetaError::new("beta graph cannot contain dangling parcel nodes"));
        }

        Ok(Self {
            schema: fixtures.graph.schema.clone(),
            node_count: fixtures.parcels.len(),
            parcel_ids: fixtures.parcels.iter().map(|parcel| parcel.id).collect(),
            parcel_names: fixtures.parcels.iter().map(|parcel| parcel.name.clone()).collect(),
            hemisphere: fixtures.parcels.iter().map(|parcel| parcel.hemi.clone()).collect(),
            adjacency,
            degree,
            non_zero_edges,
        })
    }

    pub fn apply_laplacian(&self, state: &[f32]) -> Result<Vec<f32>, BetaError> {
        if state.len() != self.node_count {
            return Err(BetaError::new("beta parcel graph dimension mismatch"));
        }
        if state.iter().any(|value| !value.is_finite()) {
            return Err(BetaError::new("beta parcel graph received non-finite state"));
        }

        Ok(self
            .adjacency
            .iter()
            .enumerate()
            .map(|(index, neighbors)| {
                let weighted_sum = neighbors
                    .iter()
                    .map(|(neighbor, weight)| weight * state[*neighbor])
                    .sum::<f32>();
                (self.degree[index] * state[index]) - weighted_sum
            })
            .collect())
    }
}
