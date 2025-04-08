use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: P, content: &str) {
    // Try to open with append mode first (file exists case)
    let file_result = OpenOptions::new()
        .append(true)
        .open(&path);

    match file_result {
        Ok(mut file) => {
            // File exists, append content
            file.write_all(content.as_bytes())
                .unwrap_or_else(|e| panic!("Failed to write to file: {}", e));
        },
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // File doesn't exist, create and write
            let mut file = File::create(path)
                .unwrap_or_else(|e| panic!("Failed to create file: {}", e));
            file.write_all(content.as_bytes())
                .unwrap_or_else(|e| panic!("Failed to write to new file: {}", e));
        },
        Err(e) => {
            // Other error cases (permissions, etc.)
            panic!("Failed to open file: {}", e);
        }
    }
}