pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::with_capacity(names.len());
    for name in names {
        let words: Vec<&str> = name.split_whitespace().collect();
        let word_initials: Vec<String> = words
            .iter()
            .filter_map(|word| {
                word.chars().next().map(|c| format!("{}.", c))
            })
            .collect();
        result.push(word_initials.join(" "));
    }
    result
}