use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    /// Create a new 1x1 matrix with zero value
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    /// Create a zero matrix with dimensions row x col
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let data = vec![vec![T::zero(); col]; row];
        Matrix(data)
    }

    /// Create an identity matrix of size n x n
    pub fn identity(n: usize) -> Matrix<T> {
        let mut data = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            data[i][i] = T::one();
        }
        Matrix(data)
    }
}
