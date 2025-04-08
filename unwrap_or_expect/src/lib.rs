pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match (server, security_level) {
        (Ok(url), Security::Unknown) => url.to_owned(), // This is a valid URL
        // We use _ for the error since we don't need its value
        (Err(_), Security::Unknown) => panic!(),
        
        (Ok(url), Security::Message) => url.to_owned(),
        (Err(_), Security::Message) => panic!("ERROR: program stops"),
        
        (Ok(url), Security::Warning) => url.to_owned(),
        (Err(_), Security::Warning) => "WARNING: check the server".to_owned(),
        
        (Ok(url), Security::NotFound) => url.to_owned(),
        (Err(e), Security::NotFound) => format!("Not found: {e}"),
        
        (Ok(url), Security::UnexpectedUrl) => panic!("{url}"),
        (Err(e), Security::UnexpectedUrl) => e.to_owned(),
    }
}