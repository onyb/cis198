/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    assert_eq!(mat1[0].len(), mat2.len());

    let rows = mat1.len();
    let cols = mat2[0].len();
    let mid = mat1[0].len();

    let mut result_matrix = vec![vec![0.; cols]; rows];

    for i in 0..rows {
        for j in 0..cols {
            for k in 0..mid {
                result_matrix[i][j] += mat1[i][k] * mat2[k][j]
            }
        }
    }
    result_matrix
}
