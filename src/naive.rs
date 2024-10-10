use std::{collections::BTreeMap, marker::PhantomData};

use crate::{index::Index, point::Point};

pub struct NaiveIndex<K, P>
where
    K: Ord,
{
    points: BTreeMap<K, P>,
    _key: PhantomData<K>,
}

impl<K, P> NaiveIndex<K, P>
where
    K: Ord,
{
    pub fn new() -> Self {
        NaiveIndex {
            points: BTreeMap::new(),
            _key: PhantomData,
        }
    }
}

impl<K, P> Index<K, P> for NaiveIndex<K, P>
where
    K: Ord + Clone,
    P: Point,
{
    fn insert(&mut self, key: K, point: P) {
        self.points.insert(key, point);
    }

    fn find(&self, query: &P, n: usize) -> Vec<(K, &P)> {
        let mut distances = self
            .points
            .iter()
            .map(|(k, p)| (k.clone(), p, query.distance(p)))
            .collect::<Vec<_>>();

        distances.sort_by(|(_, _, a), (_, _, b)| a.partial_cmp(&b).unwrap());

        distances
            .into_iter()
            .take(n)
            .map(|(k, p, _)| (k, p))
            .collect()
    }

    fn find_keys(&self, query: &P, n: usize) -> Vec<K> {
        self.find(query, n).into_iter().map(|(k, _)| k).collect()
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::point::VecPoint;

    use super::*;

    #[test]
    fn it_works() {
        let mut index = NaiveIndex::<usize, VecPoint>::new();
        index.insert(0, vec![1.0f64, 0.0f64].into());
        index.insert(1, vec![0.0f64, 1.0f64].into());
        index.insert(2, vec![0.0f64, 0.0f64].into());
        index.insert(3, vec![1.0f64, 1.0f64].into());

        assert_eq!(index.len(), 4);
        assert_eq!(
            index.find_keys(&vec![1.0f64, 0.0f64].into(), 3),
            vec![0, 3, 1]
        );
    }
}
