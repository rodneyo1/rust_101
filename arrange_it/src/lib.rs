pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<String> = phrase.split_whitespace().map(|s| s.to_string()).collect();
    
    words.sort_by_key(|word| {
        word.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u32>().unwrap()
    });
    
    words.iter_mut().for_each(|word| {
        word.retain(|c| !c.is_digit(10));
    });
    
    words.join(" ")
}