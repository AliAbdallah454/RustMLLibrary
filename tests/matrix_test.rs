use nn::math::linalg::matrix::Matrix;

// Constant broadcasting tests
#[test]
fn test_add_constant_broadcast() {

    let x = vec![
        vec![1., 2., 3.],
        vec![1., 2., 3.]
    ];
    let matrix = Matrix::from_vec(x);

    let x = vec![
        vec![2., 3., 4.],
        vec![2., 3., 4.],
    ];
    let expected_output = Matrix::from_vec(x);

    assert_eq!(expected_output, &matrix + 1.);

}

#[test]
fn test_sub_constant_broadcast() {

    let x = vec![
        vec![2., 3., 4.],
        vec![2., 3., 4.],
    ];
    let matrix = Matrix::from_vec(x);

    let x = vec![
        vec![1., 2., 3.],
        vec![1., 2., 3.],
    ];
    let expected_output = Matrix::from_vec(x);

    assert_eq!(expected_output, &matrix - 1.);
}

#[test]
fn test_div_constant_broadcast() {

    let x = vec![
        vec![2., 4., 6.],
        vec![2., 4., 6.],
    ];
    let matrix = Matrix::from_vec(x);

    let x = vec![
        vec![1., 2., 3.],
        vec![1., 2., 3.],
    ];
    let expected_output = Matrix::from_vec(x);

    assert_eq!(expected_output, &matrix / 2.);
}

// random test
#[test]
fn test_random_matrix_values_between_0_and_1() {
    let rows = 3;
    let cols = 4;

    let matrix = Matrix::random(rows, cols);

    for row in matrix.data.iter() {
        for &value in row.iter() {
            assert!(value >= 0.0 && value < 1.0, "Found value out of range: {}", value);
        }
    }
}


#[test]
fn test_mul_constant_broadcast() {

    let x = vec![
        vec![1., 2., 3.],
        vec![1., 2., 3.],
    ];
    let matrix = Matrix::from_vec(x);

    let x = vec![
        vec![2., 4., 6.],
        vec![2., 4., 6.],
    ];
    let expected_output = Matrix::from_vec(x);

    assert_eq!(expected_output, &matrix * 2.);
}


#[test]
fn test_matrix_new() {
    let matrix = Matrix::new(2, 3, 1.0);
    assert_eq!(matrix.rows, 2);
    assert_eq!(matrix.cols, 3);
    assert_eq!(matrix.data, vec![vec![1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0]]);
}

#[test]
fn test_matrix_from_vec() {
    let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let matrix = Matrix::from_vec(data.clone());
    assert_eq!(matrix.rows, 2);
    assert_eq!(matrix.cols, 2);
    assert_eq!(matrix.data, data);
}

#[test]
fn test_matrix_identity() {
    let matrix = Matrix::identity(3);
    assert_eq!(matrix.rows, 3);
    assert_eq!(matrix.cols, 3);
    assert_eq!(
        matrix.data,
        vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0]
        ]
    );
}

#[test]
fn test_matrix_transpose() {
    let matrix = Matrix::from_vec(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0]
    ]);

    let expected_transpose = Matrix::from_vec(vec![
        vec![1.0, 4.0],
        vec![2.0, 5.0],
        vec![3.0, 6.0]
    ]);

    let transposed = matrix.transpose();

    assert_eq!(transposed, expected_transpose);
}

#[test]
fn test_matrix_multiplication() {
    let matrix1 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let result = matrix1.matmul(&matrix2).unwrap();
    assert_eq!(
        result.data,
        vec![vec![19.0, 22.0], vec![43.0, 50.0]]
    );
}

#[test]
fn test_matrix_multiplication_error() {
    let matrix1 = Matrix::new(2, 3, 1.0);
    let matrix2 = Matrix::new(2, 2, 1.0);
    assert_eq!(matrix1.matmul(&matrix2), Err("Wrong dimentions"));
}

#[test]
fn test_matrix_addition() {
    let matrix1 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let result = (&matrix1 + &matrix2).unwrap();
    assert_eq!(
        result.data,
        vec![vec![6.0, 8.0], vec![10.0, 12.0]]
    );
}

#[test]
fn test_matrix_subtraction() {
    let matrix1 = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let matrix2 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let result = (&matrix1 - &matrix2).unwrap();
    assert_eq!(
        result.data,
        vec![vec![4.0, 4.0], vec![4.0, 4.0]]
    );
}

#[test]
fn test_matrix_multiplication_elementwise() {
    let matrix1 = Matrix::from_vec(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let matrix2 = Matrix::from_vec(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let result = (&matrix1 * &matrix2).unwrap();
    assert_eq!(
        result.data,
        vec![vec![5.0, 12.0], vec![21.0, 32.0]]
    );
}

#[test]
fn test_matrix_division() {
    let matrix1 = Matrix::from_vec(vec![vec![10.0, 8.0], vec![6.0, 4.0]]);
    let matrix2 = Matrix::from_vec(vec![vec![2.0, 2.0], vec![2.0, 2.0]]);
    let result = (&matrix1 / &matrix2).unwrap();
    assert_eq!(
        result.data,
        vec![vec![5.0, 4.0], vec![3.0, 2.0]]
    );
}

#[test]
fn test_broadcast_operations() {
    let large_matrix = Matrix::from_vec(vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.0, 14.0, 15.0, 16.0]
    ]);
    let small_matrix = Matrix::from_vec(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0]
    ]);
    
    let result = (&large_matrix + &small_matrix).unwrap();
    assert_eq!(
        result.data,
        vec![
            vec![2.0, 4.0, 4.0, 6.0],
            vec![8.0, 10.0, 10.0, 12.0],
            vec![10.0, 12.0, 12.0, 14.0],
            vec![16.0, 18.0, 18.0, 20.0]
        ]
    );
}