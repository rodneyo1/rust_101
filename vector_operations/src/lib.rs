// This derive macro automatically implements important traits:
// - Debug: enables printing for debugging
// - Copy/Clone: allows the type to be copied instead of moved
// - PartialEq: enables equality comparison
#[derive(Debug, Copy, Clone, PartialEq)]
// Generic struct that works with any type T
// This is a key example of Rust's powerful generics system
pub struct ThreeDVector<T> {
    pub i: T,
    pub j: T,
    pub k: T,
}

use std::ops::{Add, Sub};

// Implementation of the Add trait for vector addition
// The where clause specifies that type T must support addition
// This is an example of Rust's trait bounds and operator overloading
impl<T> Add for ThreeDVector<T>
where
    T: Add<Output = T>,
{
    // Associated type defining the result type of addition
    type Output = ThreeDVector<T>;

    // The actual implementation of vector addition
    // Thanks to Copy trait, we can take self by value
    fn add(self, rhs: ThreeDVector<T>) -> Self::Output {
        ThreeDVector {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

// Implementation of the Sub trait for vector subtraction
// Similar pattern to Add, showing Rust's consistent trait system
impl<T> Sub for ThreeDVector<T>
where
    T: Sub<Output = T>,
{
    type Output = ThreeDVector<T>;

    // Vector subtraction implementation
    // Copy trait allows for clean, intuitive syntax
    fn sub(self, rhs: ThreeDVector<T>) -> Self::Output {
        ThreeDVector {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}
