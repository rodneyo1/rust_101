pub fn initials(names :Vec<&str>) ->Vec<String>{
    let mut result = Vec::with_capacity(names.len());
    for name in names {
        let words: Vec<&str> = name.split_whitespace().collect();
        let mut word_initials = Vec::new();
        for word in words {
            if let Some(first_char) = word.chars().next() {
                word_initials.push(format!("{}.", first_char));
            }
        }
        let mut initials_string = String::new();
        for (i, initial) in word_initials.iter().enumerate() {
            initials_string.push_str(initial);
            if i < word_initials.len() - 1 {
                initials_string.push(' ');
            }
        }
        result.push(initials_string);
    }
    result
}