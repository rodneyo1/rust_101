pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            name.split_whitespace()
                .map(|part| {
                    let mut initial = String::new();
                    if let Some(c) = part.chars().next() {
                        initial.push(c);
                        initial.push('.')
                    }
                    initial
                })
                .collect::<Vec<String>>()
                .join(" ")
        })
        .collect()
}
