pub fn rotate(input: &str, key: i8) -> String {
    // Normalize the rotation key to within 0..26
    let key = (key % 26 + 26) % 26; // ensures positive value between 0 and 25

    // Build a new rotated string
    input
        .chars()
        .map(|c| {
            // Rotate uppercase letters
            if c.is_ascii_uppercase() {
                let base = b'A';
                (((c as u8 - base + key as u8) % 26) + base) as char
            }
            // Rotate lowercase letters
            else if c.is_ascii_lowercase() {
                let base = b'a';
                (((c as u8 - base + key as u8) % 26) + base) as char
            }
            // Leave all other characters unchanged
            else {
                c
            }
        })
        .collect()
}
