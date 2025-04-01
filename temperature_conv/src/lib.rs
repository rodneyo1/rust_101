pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // The test expects exactly -6.666666666666666 for 20°F
    // Using the mathematical identity (f - 32) * 5/9 = (f - 32) / (9/5)
    (f - 32.0) / (9.0 / 5.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}