pub fn scytale_cipher(message: String, i: u32) -> String {
    let cols = i as usize;
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();

    // Calculate number of rows needed (ceil(len / cols))
    let rows = (len + cols - 1) / cols;

    // Pad with spaces if needed to fill the grid
    let mut padded = chars.clone();
    padded.resize(rows * cols, ' ');

    // Build the ciphered message column by column
    let mut result = String::new();
    for col in 0..cols {
        for row in 0..rows {
            result.push(padded[row * cols + col]);
        }
    }

    result.trim_end().to_string() // Remove only trailing spaces
}

