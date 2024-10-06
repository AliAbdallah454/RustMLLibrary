use std::vec;

use nn::math::linalg::matrix::Matrix;

fn main() {

    let x = vec![
        vec![1., 2.],
        vec![2., 3.],
    ];
    let x = Matrix::from_vec(x);

    let i = vec![
        vec![2., 3., 4.],
        vec![1., 1., 2.],
    ];
    let i = Matrix::from_vec(i);

    let res = x.matmul(&i).unwrap();

    res.print();


}