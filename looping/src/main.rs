use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let correct_answer = "The letter e";
    let mut attempts = 0;

    loop {
        println!("{}", riddle);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim();

        attempts += 1;

        if input == correct_answer {
            println!("Number of trials: {}", attempts);
            break;
        }
    }
}
