pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() { // Get the first character
        // If the string is empty, return an empty string
        // If the first character is not a letter, return the string as is
        // If the first character is a letter, capitalize it and return the string
        None => String::new(),
        Some(first) => {
            let mut result = String::new();
            result.push(first.to_uppercase().next().unwrap());
            result.extend(chars);
            result
        }
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else {
            result.push(c.to_lowercase().next().unwrap());
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    input.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else {
                c.to_lowercase().next().unwrap()
            }
        })
        .collect()
}