pub enum Security {
    Unknown,
    Message,
    NotFound,
    Warning,
    UnexpectedUrl,
}

// This function simulates fetching data from a server.
// It takes a Result type as input, which can either be Ok or Err.
// Depending on the security level, it handles the Result differently.
// The function returns a String, which is the result of the operation.
// The function uses the unwrap_or, expect, and unwrap methods to handle the Result.
pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match security_level {
        Security::Unknown => server.unwrap().to_owned(), // Panics with no custom message
        Security::Message => server.expect("ERROR: program stops").to_owned(), // Panics with the message "ERROR: program stops"
        Security::Warning => server.unwrap_or("WARNING: check the server").to_owned(),
        Security::NotFound => server
            .map(String::from)
            .unwrap_or_else(|url| format!("Not found: {}", url)),
        Security::UnexpectedUrl => server.unwrap_err().to_owned(),
    }
}