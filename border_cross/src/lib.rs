#[allow(dead_code)]
pub struct Car<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
}

#[allow(dead_code)]
pub struct Truck<'a> {
    pub plate_nbr: &'a str,
    pub model: &'a str,
    pub horse_power: u32,
    pub year: u32,
    pub load_tons: u32,
}

// Define the Vehicle trait with required methods
pub trait Vehicle {
    fn model(&self) -> &str;
    fn year(&self) -> u32;
}

// Implement the Vehicle trait for Car
impl<'a> Vehicle for Car<'a> {
    fn model(&self) -> &str {
        self.model
    }

    fn year(&self) -> u32 {
        self.year
    }
}

// Implement the Vehicle trait for Truck
impl<'a> Vehicle for Truck<'a> {
    fn model(&self) -> &str {
        self.model
    }

    fn year(&self) -> u32 {
        self.year
    }
}

// This function takes a vector of trait objects (references to values that implement Vehicle)
// and collects the model name of each vehicle
pub fn all_models(list: Vec<&dyn Vehicle>) -> Vec<&str> {
    list.iter().map(|vehicle| vehicle.model()).collect()
}
