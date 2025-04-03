pub fn first_subword(mut s: String) -> String {
    if s.is_empty() {
        return s;
    }

    // Start from the second character for PascalCase check
    let split_pos = s.chars()
        .enumerate()
        .skip(1)  // Skip first character
        .find(|(_, c)| *c == '_' || c.is_uppercase())
        .map(|(i, _)| i)
        .unwrap_or(s.len());

    s.drain(..split_pos).collect()
}