use ndarray::{Array1, ArrayBase, Ix2, OwnedRepr};
pub struct PageRank {
    damping_factor: f64,
    iterations: usize,
}

impl PageRank {
    pub(crate) fn new(damping_factor: f64, iterations: usize) -> Self {
        PageRank {
            damping_factor,
            iterations,
        }
    }

    pub(crate) fn rank(&self, matrix: ArrayBase<OwnedRepr<f64>, Ix2>) -> Array1<f64> {
        let n = matrix.shape()[0];
        let mut rank = Array1::from_elem(n, 1.0 / n as f64);
        let damping = self.damping_factor;
        let teleport = (1.0 - damping) / (n as f64);
        for _ in 0..self.iterations {
            rank = matrix.dot(&rank) * damping + teleport;
        }
        rank
    }
}

