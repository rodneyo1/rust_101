pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])  // Return slice starting after the prefix
    } else {
        None  // Prefix not found
    }
}