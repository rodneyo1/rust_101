use std::ops::{Add, Sub};
use lalgebra_scalar::Scalar;

// Matrix struct definition
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

// Implement Add for Matrix
impl<T> Add for Matrix<T>
where
    T: Scalar<Item = T>,
{
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        // Ensure both matrices have the same dimensions
        if self.0.len() != rhs.0.len()
            || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len())
        {
            return None;
        }

        // Element-wise addition
        let result = self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(row_a, row_b)| {
                row_a
                    .into_iter()
                    .zip(row_b)
                    .map(|(a, b)| a + b)
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();

        Some(Matrix(result))
    }
}

// Implement Sub for Matrix
impl<T> Sub for Matrix<T>
where
    T: Scalar<Item = T>,
{
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        // Ensure both matrices have the same dimensions
        if self.0.len() != rhs.0.len()
            || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len())
        {
            return None;
        }

        // Element-wise subtraction
        let result = self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(row_a, row_b)| {
                row_a
                    .into_iter()
                    .zip(row_b)
                    .map(|(a, b)| a - b)
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();

        Some(Matrix(result))
    }
}
