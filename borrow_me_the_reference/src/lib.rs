pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len();

    // Process '+' (delete next character)
    while i > 0 {
        i -= 1;
        if chars[i] == '+' {
            chars.remove(i);
            if i < chars.len() {
                chars.remove(i);
            }
        }
    }

    // Process '-' (backspace)
    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i);
        } else {
            i += 1;
        }
    }

    *s = chars.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    // Changed to accept slice
    for element in v.iter_mut() {
        let operator_index = element.chars().position(|c| c == '+' || c == '-');
        if let Some(i) = operator_index {
            let operator = element.chars().nth(i).unwrap();
            let (left, right) = element.split_at(i);
            let x = left.parse::<i32>().expect("invalid number");
            let y = right[1..].parse::<i32>().expect("invalid number"); // Skip operator
            match operator {
                '+' => *element = (x + y).to_string(),
                '-' => *element = (x - y).to_string(), // Fixed subtraction
                _ => unreachable!(),
            }
        }
    }
}
