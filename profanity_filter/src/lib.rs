const WORD: &str ="stupid";

pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.contains(WORD) {
        Err("ERROR: illegal") //wrap the error
    } else {
        Ok(message) //returns the result
    }
}