use std::collections::BTreeMap;

use super::receptor::PARCEL_COUNT;
use super::AlphaError;

#[derive(Clone, Debug, PartialEq)]
pub struct AlphaParcelGraph {
    pub node_count: usize,
    pub adjacency: Vec<Vec<(usize, f32)>>,
    pub degree: Vec<f32>,
    pub non_zero_edges: usize,
}

impl AlphaParcelGraph {
    pub fn mock_360() -> Self {
        let mut maps = vec![BTreeMap::<usize, f32>::new(); PARCEL_COUNT];
        for node in 0..PARCEL_COUNT {
            for (offset, weight) in [(1usize, 1.0_f32), (11usize, 0.35_f32)] {
                let neighbor = (node + offset) % PARCEL_COUNT;
                maps[node].insert(neighbor, weight);
                maps[neighbor].insert(node, weight);
            }
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

        Self {
            node_count: PARCEL_COUNT,
            adjacency,
            degree,
            non_zero_edges,
        }
    }

    pub fn apply_laplacian(&self, state: &[f32]) -> Result<Vec<f32>, AlphaError> {
        if state.len() != self.node_count {
            return Err(AlphaError::new("alpha parcel graph dimension mismatch"));
        }
        if state.iter().any(|value| !value.is_finite()) {
            return Err(AlphaError::new("alpha parcel graph received non-finite state"));
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
