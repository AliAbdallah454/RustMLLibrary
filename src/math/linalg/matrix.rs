use std::vec;
use rand::Rng;

#[derive(PartialEq, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

impl Matrix {
    pub fn new(rows: usize, cols: usize, val: f64) -> Matrix {
        return Matrix {
            rows,
            cols,
            data: vec![vec![val; cols]; rows],
        }
    }

    pub fn from_vec(data: Vec<Vec<f64>>) -> Matrix {
        return Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data
        };
    }

    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        return Matrix {
            rows,
            cols,
            data: vec![vec![0.; cols]; rows]
        }
    }

    pub fn ones(rows: usize, cols: usize) -> Matrix {
        return Matrix {
            rows,
            cols,
            data: vec![vec![1.; cols]; rows]
        }
    }

    pub fn identity(dim: usize) -> Matrix {
        let mut result_matrix = Matrix {
            rows: dim,
            cols: dim,
            data: vec![vec![0.; dim]; dim],
        };

        for i in 0..dim {
                result_matrix.data[i][i] = 1.;
            }

        return result_matrix;

    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        let mut matrix= vec![vec![0.; cols]; rows];
        for i in 0..rows {
            let vector: Vec<f64> = (0..cols)
                .map(|_| rng.gen_range(0.0..1.0)).collect();
            matrix[i] = vector;
        }
        return Matrix::from_vec(matrix);
    }

    pub fn print(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.data[i][j]);
            }
            println!();
        }
    }

    pub fn transpose(&self) -> Matrix{

        let mut transposed_matrix = Matrix::new(self.cols, self.rows, 0.);

        for i in 0..self.rows {
            for j in 0..self.cols {
                transposed_matrix.data[j][i] = self.data[i][j];
            }
        }
        return transposed_matrix;
    }

    pub fn matmul(&self, other: &Self) -> Result<Matrix, &str> {
        if self.cols != other.rows {
            return Err("Wrong dimentions");
        }

        let mut result_matrix = Matrix::new(self.rows, other.cols, 0.);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result_matrix.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        return Ok(result_matrix);

    }

}