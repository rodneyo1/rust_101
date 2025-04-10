pub fn is_pangram(s: &str) -> bool {
    s.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .fold([false; 26], |mut acc, c| {
            acc[(c as usize) - ('a' as usize)] = true;
            acc
        })
        .iter()
        .all(|&present| present)
}