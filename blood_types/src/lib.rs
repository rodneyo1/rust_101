use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

// FromStr for Antigen
impl FromStr for Antigen {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(()),
        }
    }
}

// FromStr for RhFactor
impl FromStr for RhFactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err(()),
        }
    }
}

// FromStr for BloodType
impl FromStr for BloodType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (antigen_str, rh_str) = if s.ends_with('+') {
            (&s[..s.len()-1], "+")
        } else if s.ends_with('-') {
            (&s[..s.len()-1], "-")
        } else {
            return Err(());
        };

        let antigen = antigen_str.parse()?;
        let rh_factor = rh_str.parse()?;
        Ok(BloodType { antigen, rh_factor })
    }
}

// Custom Debug implementation
impl fmt::Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let antigen_str = match self.antigen {
            Antigen::A => "A",
            Antigen::B => "B",
            Antigen::AB => "AB",
            Antigen::O => "O",
        };
        let rh_str = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };
        write!(f, "{}{}", antigen_str, rh_str)
    }
}

// Custom Ord implementation
impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}

// Main logic for compatibility
impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Rh compatibility: positive can receive from both, negative only from negative
        let rh_ok = match (&self.rh_factor, &other.rh_factor) {
            (RhFactor::Positive, _) => true,
            (RhFactor::Negative, RhFactor::Negative) => true,
            _ => false,
        };

        let antigen_ok = match self.antigen {
            Antigen::O => matches!(other.antigen, Antigen::O),
            Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
            Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
            Antigen::AB => true,
        };

        rh_ok && antigen_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        all_blood_types()
            .into_iter()
            .filter(|b| self.can_receive_from(b))
            .collect()
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        all_blood_types()
            .into_iter()
            .filter(|b| b.can_receive_from(self))
            .collect()
    }
}

// Helper function
fn all_blood_types() -> Vec<BloodType> {
    use Antigen::*;
    use RhFactor::*;

    let antigens = vec![O, A, B, AB];
    let rh_factors = vec![Negative, Positive];

    let mut result = vec![];
    for antigen in antigens {
        for rh in &rh_factors {
            result.push(BloodType {
                antigen: antigen.clone(),
                rh_factor: rh.clone(),
            });
        }
    }
    result
}
