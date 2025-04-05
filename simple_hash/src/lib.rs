use std::collections::HashMap;

// This function takes a slice of words and returns a HashMap where the keys are the words
// and the values are the number of times each word appears in the slice.
pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut hash = HashMap::with_capacity(words.len() / 2);

    words
        .iter()
        .copied()
        .for_each(|w| *hash.entry(w).or_default() += 1);

    hash
}

// This function takes a reference to a HashMap where the keys are words and the values are their frequencies
pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}