use super::matrix::Matrix;
use std::ops::{Add, Div, Mul, Sub};


impl Matrix {
    
    fn broadcast<F>(&self, rhs: &Self, operation: F) -> Result<Matrix, &'static str>
        where 
            F: Fn(f64, f64) -> f64,
    {
        let row_div = self.rows as f64 / rhs.rows as f64;
        let col_div = self.cols as f64 / rhs.cols as f64;

        let mut new_matrix = Matrix::new(self.rows, self.cols, 0.);

        if row_div.fract() != 0.0 || col_div.fract() != 0.0 {
            return Err("Dimention error: Broadcasting cannot be applied");
        }

        if  row_div.fract() == 0.0 && col_div.fract() == 0.0 {
            for i in 0..row_div as usize {
                for j in 0..col_div as usize {
                    for k in 0..rhs.rows {
                        for z in 0..rhs.cols {
                            new_matrix.data[k + rhs.rows*i][z + rhs.cols*j] = 
                                operation(self.data[k + rhs.rows*i][z + rhs.cols*j], rhs.data[k][z]);
                        }
                    }
                }
            }
        }
        return Ok(new_matrix);
    }

    fn broadcast_constant<F>(&self, rhs: f64, operation: F) -> Result<Matrix, &'static str>
        where 
            F: Fn(f64, f64) -> f64,
    {
        let unit_matrix = Matrix::new(1, 1, rhs);
        return self.broadcast(&unit_matrix, operation);
    }
}

impl Add for &Matrix {
    type Output = Result<Matrix, &'static str>;

    fn add(self, rhs: Self) -> Self::Output {
        return self.broadcast(rhs, |x: f64, y:f64| x + y);
    }
}

impl Sub for &Matrix {
    type Output = Result<Matrix, &'static str>;

    fn sub(self, rhs: Self) -> Self::Output {
        return self.broadcast(rhs, |x: f64, y:f64| x - y);
    }
}

impl Div for &Matrix {
    type Output = Result<Matrix, &'static str>;

    fn div(self, rhs: Self) -> Self::Output {
        return self.broadcast(rhs, |x: f64, y:f64| x / y);
    }
}

impl Mul for &Matrix {
    type Output = Result<Matrix, &'static str>;

    fn mul(self, rhs: Self) -> Self::Output {
        return self.broadcast(rhs, |x: f64, y:f64| x * y);
    }
}



impl Add<f64> for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: f64) -> Self::Output {
        return self.broadcast_constant(rhs, |x: f64, y: f64| x + y)
            .expect("Constant broadcasting should never fail");
    }
}

impl Sub<f64> for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Self::Output {
        return self.broadcast_constant(rhs, |x: f64, y: f64| x - y)
            .expect("Constant broadcasting should never fail");
    }
}

impl Div<f64> for &Matrix {
    type Output = Matrix;
    fn div(self, rhs: f64) -> Self::Output {
        return self.broadcast_constant(rhs, |x: f64, y: f64| x / y)
            .expect("Constant broadcasting should never fail");
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output {
        return self.broadcast_constant(rhs, |x: f64, y: f64| x * y)
            .expect("Constant broadcasting should never fail");
    }
}