// Enum representing Roman numerals - a perfect example of using
// enums for domain modeling and type safety
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,  // Special case for zero
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

// NewType pattern: Wrapping a Vec in a struct provides type safety
// and encapsulation while allowing custom implementations
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

use crate::RomanDigit::*;
use std::convert::From;

// Converting from u32 to RomanDigit demonstrates pattern matching
// and type conversion for individual symbols
impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => panic!("Unsupported digit"),
        }
    }
}

// Implementation of From trait shows how to create clean,
// idiomatic conversion APIs in Rust
impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        // Special case handling demonstrates defensive programming
        if num == 0 {
            return RomanNumber(vec![Nulla]);
        }

        // Vec<RomanDigit> demonstrates ownership and heap allocation
        let mut result = Vec::new();
        
        // Structured data for conversion logic
        let mapping = [
            (1000, M),
            (900, C), (900, M), // CM
            (500, D),
            (400, C), (400, D), // CD
            (100, C),
            (90, X), (90, C),   // XC
            (50, L),
            (40, X), (40, L),   // XL
            (10, X),
            (9, I), (9, X),     // IX
            (5, V),
            (4, I), (4, V),     // IV
            (1, I),
        ];

        // Complex control flow showing Rust's powerful
        // iteration and mutation patterns
        let mut i = 0;
        while i < mapping.len() {
            let (val, digit) = mapping[i];
            if i + 1 < mapping.len() && mapping[i].0 == mapping[i + 1].0 {
                if num >= val {
                    result.push(mapping[i].1);
                    result.push(mapping[i + 1].1);
                    num -= val;
                }
                i += 2;
            } else if num >= val {
                result.push(digit);
                num -= val;
            } else {
                i += 1;
            }
        }

        RomanNumber(result)
    }
}
