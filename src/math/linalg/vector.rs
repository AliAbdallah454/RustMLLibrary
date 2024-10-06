use super::matrix::{self, Matrix};

struct Vector {
    dim: usize,
    data: Vec<f64>,
}

impl Vector {
    
    pub fn new(v: Vec<f64>) -> Vector {
        return Vector {
            dim: v.len(),
            data: v,
        };
    }
}