use std::ops::{Add, Sub, Mul, Div};

// Trait Scalar with common numeric operations and constants
pub trait Scalar:
    Sized + // must be a sized type
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Copy // allow simple pass-by-value
{
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

// Implement Scalar for integer and float primitives

macro_rules! impl_scalar {
    ($($t:ty),*) => {
        $(
            impl Scalar for $t {
                type Item = $t;

                fn zero() -> Self::Item {
                    0 as $t
                }

                fn one() -> Self::Item {
                    1 as $t
                }
            }
        )*
    }
}

// Apply it to requested types
impl_scalar!(u32, u64, i32, i64, f32, f64);
