use std::ops::Add;
use std::fmt::Debug;

use lalgebra_scalar::Scalar;

// Multiple trait bounds showcase Rust's powerful type system:
// - Scalar for linear algebra operations
// - Standard traits for utility functions
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar<Item = T>>(pub Vec<T>);

// Operator overloading with Option<T> for safe operations
// Returns None if vectors have different lengths
impl<T: Scalar<Item = T>> Add for Vector<T> {
    type Output = Option<Self>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let result: Vec<T> = self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        Some(Vector(result))
    }
}

// Implementation combining Scalar trait operations
// with linear algebra computations
impl<T: Scalar<Item = T>> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    // Safe dot product using Option<T> for error handling
    // Shows how Rust encourages explicit error handling
    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut sum = T::zero();

        for (a, b) in self.0.iter().zip(other.0.iter()) {
            sum = sum + (*a * *b);
        }

        Some(sum)
    }
}
