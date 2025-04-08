// Imports the `Utc` struct from the `chrono` crate for working with UTC time.
pub use chrono::Utc;

/// Represents an error that can occur during form validation.
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    /// The field name and its value that caused the error.
    pub form_values: (&'static str, String),
    /// The date and time when the error occurred.
    pub date: String,
    /// A static string describing the error.
    pub err: &'static str,
}

impl FormError {
    /// Creates a new `FormError` instance.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the form field where the error occurred.
    /// * `field_value` - The value of the form field that caused the error.
    /// * `err` - A static string describing the error.
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            // Gets the current UTC time and formats it as "YYYY-MM-DD HH:MM:SS".
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

/// Represents a simple form with a name and password.
#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    /// The name entered in the form.
    pub name: String,
    /// The password entered in the form.
    pub password: String,
}

impl Form {
    /// Validates the form data.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the form data is valid.
    /// * `Err(FormError)` if any validation rule is violated.
    pub fn validate(&self) -> Result<(), FormError> {
        // Checks if the name field is empty.
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        // Helper closure to create a `FormError` for password validation failures.
        let password_error =
            |m: &'static str| Err(FormError::new("password", self.password.clone(), m));

        // Checks if the password length is less than 8 characters.
        if self.password.len() < 8 {
            return password_error("Password should be at least 8 characters long");
        } else if !(
            // Checks if the password contains at least one ASCII digit.
            self.password.chars().any(|c| c.is_ascii_digit())
            // Checks if the password contains at least one ASCII alphabetic character.
            && self.password.chars().any(|c| c.is_ascii_alphabetic())
            // Checks if the password contains at least one ASCII punctuation character.
            && self.password.chars().any(|c| c.is_ascii_punctuation())
        )
        {
            return password_error(
                "Password should be a combination of ASCII numbers, letters and symbols",
            );
        }

        // If all validation checks pass, return Ok.
        Ok(())
    }
}