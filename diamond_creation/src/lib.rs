pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }

    let n = (c as u8 - b'A' + 1) as usize;
    let mut diamond = Vec::with_capacity(2 * n - 1);

    // Generate the top half of the diamond
    for i in 0..n {
        let current_char = (b'A' + i as u8) as char;
        let outer_spaces = " ".repeat(n - 1 - i);
        if current_char == 'A' {
            diamond.push(format!("{outer_spaces}A{outer_spaces}"));
        } else {
            let inner_spaces = " ".repeat(2 * i - 1);
            diamond.push(format!("{outer_spaces}{current_char}{inner_spaces}{current_char}{outer_spaces}"));
        }
    }

    // Generate the bottom half of the diamond (mirror of the top half excluding the middle line)
    for i in (0..n-1).rev() {
        let current_char = (b'A' + i as u8) as char;
        let outer_spaces = " ".repeat(n - 1 - i);
        if current_char == 'A' {
            diamond.push(format!("{outer_spaces}A{outer_spaces}"));
        } else {
            let inner_spaces = " ".repeat(2 * i - 1);
            diamond.push(format!("{outer_spaces}{current_char}{inner_spaces}{current_char}{outer_spaces}"));
        }
    }

    diamond
}