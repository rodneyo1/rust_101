// Define a public error type for incorrect cipher results
#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,  // Stores the correct cipher text for error reporting
}

/// Performs Atbash cipher verification
/// Returns Ok(()) if the ciphered text matches the expected transformation of original
/// Returns Err(CipherError) with the expected cipher if verification fails
pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    // Generate the expected cipher by transforming each character
    let expected = original.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            // Determine case (lowercase or uppercase)
            let case = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            // Calculate position in alphabet (0-25)
            let offset = c as u8 - case;
            // Mirror the character in the alphabet:
            // a ↔ z, b ↔ y, c ↔ x, etc.
            (case + (25 - offset)) as char
        } else {
            // Non-alphabetic characters remain unchanged
            c
        }
    }).collect::<String>();  // Combine characters into a string

    // Compare generated cipher with provided cipher
    if expected == ciphered {
        Ok(())  // Return success if they match
    } else {
        Err(CipherError { expected })  // Return error with expected cipher if they don't
    }
}