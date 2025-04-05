use std::collections::HashMap;

// This function takes a slice of integers and returns the mean, median, and mode of the integers.
pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum(); // Calculate the sum of the integers in the slice
    if list.is_empty() {
        return 0.0; // Return 0.0 if the slice is empty to avoid division by zero
    }
    sum as f64 / list.len() as f64
}

// This function takes a slice of integers and returns the median of the integers.
pub fn median(list: &[i32]) -> i32 {
    let mut sorted_list = list.to_vec();
    sorted_list.sort_unstable(); // Sort the list in place
    if sorted_list.is_empty() {
        return 0; // Return 0 if the list is empty
    }
    let len = sorted_list.len();
    if len % 2 == 1 {
        sorted_list[len / 2]
    } else {
        (sorted_list[len / 2 - 1] + sorted_list[len / 2]) / 2 // Return the average of the two middle elements
    }
}

// This function takes a slice of integers and returns the mode of the integers.
pub fn mode(list: &[i32]) -> i32 {
    let mut frequency_map = HashMap::new(); // Create a HashMap to store the frequency of each integer
    // Iterate over the list and count the frequency of each integer
    for &num in list {
        *frequency_map.entry(num).or_insert(0) += 1;
    }
    frequency_map
        .into_iter()
        .max_by_key(|&(num, count)| (count, num)) // Find the integer with the highest frequency
        // If there are multiple integers with the same frequency, return the smallest one
        .map(|(num, _)| num) // Extract the integer from the tuple
        // If the list is empty, return 0
        .unwrap_or(0)
}