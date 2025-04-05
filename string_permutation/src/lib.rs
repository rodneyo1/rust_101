use std::collections::HashMap;

// This function takes two strings and checks if one is a permutation of the other.
// A permutation is a rearrangement of the letters of a word to form another word.
// For example, "listen" is a permutation of "silent".
// The function first checks if the lengths of the two strings are equal.
// If they are not, it returns false.
// If they are equal, it creates a HashMap to count the occurrences of each character in the first string.
// It then iterates over the characters in the first string, incrementing the count for each character.
// Next, it iterates over the characters in the second string, decrementing the count for each character.
// If the count for any character becomes negative, it means that the second string has more occurrences of that character than the first string,
// so it returns false.
pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut char_counts = HashMap::new();

    for c in s1.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    for c in s2.chars() {
        let count = char_counts.entry(c).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }

    char_counts.values().all(|&count| count == 0)
}