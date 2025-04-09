use expected_variable::*;

fn main() {
    match expected_variable("On_Point", "on_point") {
        Some(result) => println!("{} close to it", result),
        None => println!("Not similar enough"),
    }

    match expected_variable("soClose", "so_close") {
        Some(result) => println!("{} close to it", result),
        None => println!("Not similar enough"),
    }

    match expected_variable("something", "something_completely_different") {
        Some(result) => println!("{} close to it", result),
        None => println!("None"),
    }

    match expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch") {
        Some(result) => println!("{} close to it", result),
        None => println!("Not similar enough"),
    }
}