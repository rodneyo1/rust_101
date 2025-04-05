use arrays::*;

fn main() {
    let a: Vec<i32> = (1..=10).collect();  // Creates Vec [1, 2, ..., 10]
    let b = [5; 10];  // Creates array [5, 5, ..., 5] with 10 elements

    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!("The sum of the elements in {:?} is {}", b, sum(&b));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}