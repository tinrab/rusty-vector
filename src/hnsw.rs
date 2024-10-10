use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use crate::{index::Index, point::Point};

struct HnswNode<K, P>
where
    K: Ord,
{
    // key: K,
    point: P,
    connections: BTreeMap<usize, BTreeSet<K>>,
}

pub struct HnswIndex<K, P>
where
    K: Ord,
{
    nodes: BTreeMap<K, HnswNode<K, P>>,
    max_level: usize,
    construction_expansion_factor: usize,
    normalization_factor: f64,
    rng: StdRng,
}

impl<K, P> HnswIndex<K, P>
where
    K: Ord + Clone,
    P: Point,
{
    /// Create a new HNSW index with the given parameters.
    ///
    /// The expansion factor determines how many nearest neighbors to consider when inserting a new node.
    pub fn new(construction_expansion_factor: usize, normalization_factor: f64) -> Self {
        HnswIndex {
            nodes: BTreeMap::new(),
            max_level: 0,
            construction_expansion_factor,
            normalization_factor,
            rng: StdRng::seed_from_u64(0u64),
        }
    }

    pub fn with_rng(
        construction_expansion_factor: usize,
        normalization_factor: f64,
        rng: StdRng,
    ) -> Self {
        HnswIndex {
            nodes: BTreeMap::new(),
            max_level: 0,
            construction_expansion_factor,
            normalization_factor,
            rng,
        }
    }

    pub fn set_rng(&mut self, rng: StdRng) {
        self.rng = rng;
    }

    fn search_layer(
        &self,
        entry: Option<K>,
        query: &P,
        search_expansion_factor: usize,
        level: usize,
    ) -> Vec<(K, &P)> {
        let entry_point = match entry {
            Some(k) => k,
            None => {
                let entry_point = self.nodes.keys().next().unwrap();
                entry_point.clone()
            }
        };

        let mut candidates = BTreeSet::new();
        candidates.insert(entry_point.clone());
        let mut visited = BTreeSet::new();
        visited.insert(entry_point.clone());
        let mut results = vec![(entry_point.clone(), &self.nodes[&entry_point].point)];

        while !candidates.is_empty() {
            let current = candidates
                .iter()
                .min_by(|a, b| {
                    self.nodes[a]
                        .point
                        .distance(query)
                        .partial_cmp(&self.nodes[b].point.distance(query))
                        .unwrap()
                })
                .cloned()
                .unwrap();
            candidates.remove(&current);

            if results.len() >= search_expansion_factor
                && self.nodes[&results[results.len() - 1].0]
                    .point
                    .distance(query)
                    < self.nodes[&current].point.distance(query)
            {
                break;
            }

            if let Some(connections) = self.nodes[&current].connections.get(&level) {
                for neighbor_key in connections.iter() {
                    if !visited.contains(neighbor_key) {
                        visited.insert(neighbor_key.clone());
                        candidates.insert(neighbor_key.clone());
                        let neighbor = &self.nodes[neighbor_key];
                        if results.len() < search_expansion_factor
                            || neighbor.point.distance(query)
                                < self.nodes[&results[results.len() - 1].0]
                                    .point
                                    .distance(query)
                        {
                            results.push((neighbor_key.clone(), &neighbor.point));
                            results.sort_by(|(_, a), (_, b)| {
                                a.distance(query).partial_cmp(&b.distance(query)).unwrap()
                            });
                            if results.len() > search_expansion_factor {
                                results.pop();
                            }
                        }
                    }
                }
            }
        }

        results
    }
}

impl<K, P> Index<K, P> for HnswIndex<K, P>
where
    K: Ord + Clone,
    P: Point,
{
    fn insert(&mut self, key: K, vector: P) {
        self.nodes.insert(
            key.clone(),
            HnswNode {
                // key: key.clone(),
                point: vector,
                connections: BTreeMap::new(),
            },
        );

        let level =
            (-self.rng.gen::<f64>().ln() * self.normalization_factor as f64).floor() as usize;
        if level > self.max_level {
            self.max_level = level;
        }

        let mut current_node = None;
        for l in (0..=level.min(self.max_level)).rev() {
            let mut neighbors = self.search_layer(
                current_node,
                &self.nodes[&key].point,
                self.construction_expansion_factor,
                l,
            );
            neighbors = self.select_neighbors(&neighbors, self.m, &self.nodes[&id].vector);

            self.nodes
                .get_mut(&id)
                .unwrap()
                .connections
                .insert(l, neighbors.iter().cloned().collect());

            for &neighbor_id in &neighbors {
                self.nodes
                    .get_mut(&neighbor_id)
                    .unwrap()
                    .connections
                    .entry(l)
                    .or_insert_with(HashSet::new)
                    .insert(id);
            }

            current_node = neighbors.first().cloned();
        }
    }

    fn find(&self, query: &P, n: usize) -> Vec<(K, &P)> {
        let mut current_node = self.nodes.keys().next().unwrap().clone();
        for level in (1..=self.max_level).rev() {
            let neighbors = self.search_layer(Some(current_node), query, n, level);
            current_node = neighbors[0].0.clone();
        }
        self.search_layer(Some(current_node), query, 1, 0)
    }

    fn find_keys(&self, query: &P, n: usize) -> Vec<K> {
        self.find(query, n).into_iter().map(|(k, _)| k).collect()
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::point::VecPoint;

    use super::*;

    #[test]
    fn it_works() {
        let mut index = HnswIndex::<usize, VecPoint>::new(10, 1.0f64);
        index.insert(0, vec![1.0f64, 0.0f64].into());
        index.insert(1, vec![0.0f64, 1.0f64].into());
        index.insert(2, vec![0.0f64, 0.0f64].into());
        index.insert(3, vec![1.0f64, 1.0f64].into());

        assert_eq!(index.len(), 4);
        dbg!(index.find_keys(&vec![1.0f64, 0.0f64].into(), 3));
        // assert_eq!(
        //     index.find_keys(&vec![1.0f64, 0.0f64].into(), 3),
        //     vec![0, 3, 1]
        // );
    }
}
