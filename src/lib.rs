pub mod math;
pub mod utils;

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use math::linalg::matrix::Matrix;

//     #[test]
//     fn test_add_same_size_matrices() {
//         let matrix1 = Matrix {
//             rows: 2,
//             cols: 2,
//             data: vec![
//                 vec![1.0, 2.0],
//                 vec![3.0, 4.0],
//             ],
//         };

//         let matrix2 = Matrix {
//             rows: 2,
//             cols: 2,
//             data: vec![
//                 vec![5.0, 6.0],
//                 vec![7.0, 8.0],
//             ],
//         };

//         let expected_result = Matrix {
//             rows: 2,
//             cols: 2,
//             data: vec![
//                 vec![6.0, 8.0],
//                 vec![10.0, 12.0],
//             ],
//         };

//         let result = &matrix1 + &matrix2;
//         let result = result.unwrap();
//         assert_eq!(result, expected_result);
//     }

//     #[test]
//     fn test_add_tiled_matrices() {
//         let matrix1 = Matrix {
//             rows: 4,
//             cols: 4,
//             data: vec![
//                 vec![1.0, 2.0, 3.0, 4.0],
//                 vec![5.0, 6.0, 7.0, 8.0],
//                 vec![9.0, 10.0, 11.0, 12.0],
//                 vec![13.0, 14.0, 15.0, 16.0],
//             ],
//         };

//         let matrix2 = Matrix {
//             rows: 2,
//             cols: 2,
//             data: vec![
//                 vec![1.0, 1.0],
//                 vec![1.0, 1.0],
//             ],
//         };

//         let expected_result = Matrix {
//             rows: 4,
//             cols: 4,
//             data: vec![
//                 vec![2.0, 3.0, 4.0, 5.0],
//                 vec![6.0, 7.0, 8.0, 9.0],
//                 vec![10.0, 11.0, 12.0, 13.0],
//                 vec![14.0, 15.0, 16.0, 17.0],
//             ],
//         };

//         let result = &matrix1 + &matrix2;
//         let result = result.unwrap();

//         assert_eq!(result, expected_result);
//     }

//     #[test]
//     fn test_add_incompatible_matrices() {
//         let matrix1 = Matrix {
//             rows: 3,
//             cols: 3,
//             data: vec![
//                 vec![1.0, 2.0, 3.0],
//                 vec![4.0, 5.0, 6.0],
//                 vec![7.0, 8.0, 9.0],
//             ],
//         };

//         let matrix2 = Matrix {
//             rows: 2,
//             cols: 2,
//             data: vec![
//                 vec![1.0, 1.0],
//                 vec![1.0, 1.0],
//             ],
//         };

//         let result = &matrix1 + &matrix2;
//         let result = result.unwrap();

//         assert_eq!(result.data.len(), matrix1.rows);
//     }

//     #[test]
//         fn add_broadcast_test_1() {
//             let mut v = Matrix::new(2, 4, 0.);
//             let one = Matrix::new(1, 1, 1.);
    
//             v.data[0][0] = 1.;
//             v.data[0][1] = 2.;
//             v.data[0][2] = 3.;
    
//             v.data[1][0] = 4.;
//             v.data[1][1] = 5.;
//             v.data[1][2] = 6.;
    
//             let mut m = Matrix::new(2, 4, 0.);
    
//             m.data[0][0] = 2.;
//             m.data[0][1] = 3.;
//             m.data[0][2] = 4.;
//             m.data[0][3] = 1.;
    
//             m.data[1][0] = 5.;
//             m.data[1][1] = 6.;
//             m.data[1][2] = 7.;
//             m.data[1][3] = 1.;
    
//             let a = &v + &one;
//             let a = a.unwrap();

//             assert_eq!(a, m);
    
//         }

// }