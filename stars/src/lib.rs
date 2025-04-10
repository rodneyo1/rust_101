pub fn stars(n: u32) -> String {
    // Calculate 2^n
    let num_stars = 2u32.pow(n);
    // Create a string with that many stars
    "*".repeat(num_stars as usize)
}