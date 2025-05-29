use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");

    // Variable immutable by default
    // Type inference
    let secret_number = rand::thread_rng().gen_range(1..100); // 1..100 range from 1 inclusive to 100 exclusive

    loop {
        println!("Please input your guess.");

        // mut keyword for mutable variable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // Passing a mutable reference to the function
            .expect("Failed to read line.");

        // Shadowing
        // ': Type' specify the wanted type to help the parse() function
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ match-all value, here all error values
            Err(_) => {
                println!("Please enter valid number.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
