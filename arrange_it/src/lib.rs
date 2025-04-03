pub fn arrange_phrase(phrase: &str) -> String {
    // Split the phrase into words
    let words = phrase.split_whitespace();
    
    // Count the number of words to properly size our vector
    let word_count = words.clone().count();
    
    // Create a vector with capacity for all words to avoid reallocation
    let mut indexed_words = Vec::with_capacity(word_count);
    
    // Process each word to extract position and store in the vector
    for word in words {
        // Find the digit in the word
        let mut position = 0;
        let mut actual_word = String::with_capacity(word.len());
        
        for c in word.chars() {
            if c.is_digit(10) {
                position = c.to_digit(10).unwrap() as usize;
            } else {
                actual_word.push(c);
            }
        }
        
        // Store the word and its position (index is 1-based in the problem)
        indexed_words.push((position, actual_word));
    }
    
    // Sort words by their position
    indexed_words.sort_by_key(|&(pos, _)| pos);
    
    // Build the result string with exact capacity needed
    let total_length = indexed_words.iter()
        .map(|(_, word)| word.len())
        .sum::<usize>() + word_count - 1; // Add spaces between words
    
    let mut result = String::with_capacity(total_length);
    
    // Append words in sorted order
    for (i, (_, word)) in indexed_words.iter().enumerate() {
        result.push_str(word);
        if i < word_count - 1 {
            result.push(' ');
        }
    }
    
    result
}