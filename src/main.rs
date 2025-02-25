use rand::prelude::*;
use std::io;

fn main() {
    let list = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut attempt: i16 = 0; // Counter for attempts
    let mut attempts_record: Vec<String> = Vec::new(); // Vector to store all attempts

    loop {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..list.len());
        let random_number_selected = list[index];

        println!("Attempts: {}", attempt);
        println!("Please enter your guess: ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let random_guess = input.trim().to_string();

                if !list.contains(&random_guess.as_str()) {
                    println!("Invalid Number! Please enter from 1 to 9: ");
                    continue;
                }

                attempt += 1; // Increment attempt counter
                attempts_record.push(random_guess.clone()); // Store attempt

                if guess(&random_guess, &random_number_selected) {
                    println!("Correct Guess! You did it in {} attempts.", attempt);
                    println!("Your attempts: {:?}", attempts_record);
                    break;
                } else {
                    println!("Wrong guess! The correct number was {}, try again.", random_number_selected);
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                continue;
            }
        }
    }
}

fn guess(guess: &str, actual: &str) -> bool {
    guess == actual
}
