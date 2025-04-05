use std::collections::HashMap;

// This function takes a HashMap where the keys are strings and the values are integers.
// It returns the maximum value from the HashMap, or 0 if there are no positive values.
// The function uses the filter method to filter out values that are less than or equal to 0,
// and then uses the max method to find the maximum value among the remaining values.
// The max method returns an Option, so we use the copied method to convert it to an i32.
// If there are no positive values, we return 0 using unwrap_or(0).
// The function is generic and can work with any type of HashMap that has string keys and integer values.
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    h.values().filter(|&&v| v > 0).max().copied().unwrap_or(0)
}