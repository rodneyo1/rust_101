use json::JsonValue;
use std::str::FromStr;

#[derive(Debug)]
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;
    let mut total_fats = 0.0;

    for food in foods {
        // Parse kcal value from the second element in calories array
        let kcal_str = food.calories[1].replace("kcal", "");
        let kcal = f64::from_str(&kcal_str).unwrap_or(0.0);
        
        // Add values multiplied by portions
        total_cals += kcal * food.nbr_of_portions;
        total_carbs += food.carbs * food.nbr_of_portions;
        total_proteins += food.proteins * food.nbr_of_portions;
        total_fats += food.fats * food.nbr_of_portions;
    }

    // Round to two decimal places (or one if it ends with .0)
    let round = |value: f64| {
        let rounded = (value * 100.0).round() / 100.0;
        if rounded.fract() == 0.0 {
            (value * 10.0).round() / 10.0
        } else {
            rounded
        }
    };

    json::object! {
        "cals" => round(total_cals),
        "carbs" => round(total_carbs),
        "proteins" => round(total_proteins),
        "fats" => round(total_fats),
    }
}