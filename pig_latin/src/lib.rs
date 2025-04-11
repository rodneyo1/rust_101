pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    // Helper function to find the index of the first vowel
    fn first_vowel_index(word: &str) -> usize {
        for (i, ch) in word.char_indices() {
            if ['a', 'e', 'i', 'o', 'u'].contains(&ch) {
                return i;
            }
        }
        word.len() // fallback if no vowel
    }

    let word = text;

    if word.is_empty() {
        return String::new();
    }

    let chars: Vec<char> = word.chars().collect();

    // If it starts with a vowel
    if vowels.contains(&chars[0]) {
        return format!("{}ay", word);
    }

    // Check for consonant followed by "qu"
    if chars.len() >= 3 && !vowels.contains(&chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
        let rest: String = chars[3..].iter().collect();
        let start: String = chars[..3].iter().collect();
        return format!("{}{}ay", rest, start);
    }

    // Find the first vowel index
    let index = first_vowel_index(word);
    let (start, end) = word.split_at(index);
    format!("{}{}ay", end, start)
}
