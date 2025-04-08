use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, desc: &str) -> Self {
        Flag {
            short_hand: format!("-{}", name.chars().next().unwrap()), // Take first char for short flag
            long_hand: format!("--{}", name),
            desc: desc.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>, // Using String as key for simplicity
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        // Insert both short and long versions into the HashMap
        self.flags.insert(flag.short_hand.clone(), func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        // Look up the callback function
        let callback = self.flags.get(input)
            .ok_or_else(|| format!("flag {} not found", input))?;

        // Check we have enough arguments
        if argv.len() < 2 {
            return Err("not enough arguments".to_string());
        }

        // Execute the callback and convert any ParseFloatError to String
        callback(argv[0], argv[1])
            .map_err(|e| e.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num: f64 = a.parse()?;
    let b_num: f64 = b.parse()?;
    Ok((a_num / b_num).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a_num: f64 = a.parse()?;
    let b_num: f64 = b.parse()?;
    Ok((a_num % b_num).to_string())
}