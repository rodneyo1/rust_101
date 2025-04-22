// Define a public struct called Person with a lifetime parameter 'a
// This tells Rust that the Person struct can't outlive the string it references
#[derive(Debug)]
pub struct Person<'a> {
    pub name: &'a str,  // &'a str is a string slice with lifetime 'a
    pub age: u8,       // u8 is appropriate for age as it's typically 0-120
}

// Implementation block for the Person struct
// We need to specify the lifetime here too
impl<'a> Person<'a> {
    // Associated function 'new' that takes a name (string slice with lifetime 'a)
    // and returns a Person with the same lifetime
    pub fn new(name: &'a str) -> Person<'a> {
        // Create and return a new Person instance
        Person {
            name: name,  // Set the name field to the provided name
            age: 0,       // Initialize age to 0 as specified
        }
    }
}