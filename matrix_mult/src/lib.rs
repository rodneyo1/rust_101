use std::ops::Mul;
use lalgebra_scalar::Scalar; // or `use super::Scalar;` depending on your file structure

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    // Returns number of columns (assumes non-jagged matrix)
    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    // Returns number of rows
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    // Returns the nth row (clone for owned return)
    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    // Returns the nth column by collecting nth element of each row
    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}

// Matrix multiplication
impl<T: Scalar<Item = T>> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let rows_lhs = self.number_of_rows();
        let cols_lhs = self.number_of_cols();
        let rows_rhs = rhs.number_of_rows();
        let cols_rhs = rhs.number_of_cols();

        // Check if inner dimensions match
        if cols_lhs != rows_rhs {
            return None;
        }

        let mut result = vec![vec![T::zero(); cols_rhs]; rows_lhs];

        for i in 0..rows_lhs {
            for j in 0..cols_rhs {
                for k in 0..cols_lhs {
                    result[i][j] = result[i][j].clone()
                        + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
            }
        }

        Some(Matrix(result))
    }
}
