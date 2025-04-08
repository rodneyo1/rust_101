// This is a library for working with arrays in Rust.
pub fn sum(a: &[i32]) -> i32 {
    a.iter().sum() // Calculate the sum of the integers in the array
}

// This function returns an array of 32 integers, each initialized to 10.  
pub fn thirtytwo_tens() -> [i32; 32] {
    [10; 32] // Create an array of 32 integers, each initialized to 10
}