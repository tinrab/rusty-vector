use std::ops::{Deref, DerefMut};

use simsimd::SpatialSimilarity;

pub trait Point {
    fn distance(&self, other: &Self) -> f64;
}

#[derive(Debug, Clone)]
pub struct VecPoint(Vec<f64>);

impl VecPoint {
    pub fn new() -> Self {
        VecPoint(Vec::new())
    }
}

impl From<Vec<f64>> for VecPoint {
    fn from(v: Vec<f64>) -> Self {
        VecPoint(v)
    }
}

impl Deref for VecPoint {
    type Target = Vec<f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VecPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Point for VecPoint {
    fn distance(&self, other: &Self) -> f64 {
        f64::cosine(&self.0, &other.0).expect("vectors must be of the same length")
    }
}
