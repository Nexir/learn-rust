use rand::Rng;
use std::{cmp::Ordering, io};

const MAX_TRIES: u8 = 10;

fn main() {
    println!("Guess the number!");

    // Variable immutable by default
    // Type inference
    let secret_number = rand::thread_rng().gen_range(1..100); // 1..100 range from 1 inclusive to 100 exclusive
    let mut nb_guesses = 0;

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

        nb_guesses += 1;

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You found the secret number in {nb_guesses} guesses.");
                break;
            }
        }

        if nb_guesses >= MAX_TRIES {
            println!("You lose!");
            break;
        } else {
            println!("You have {} guesses left.", MAX_TRIES - nb_guesses)
        }
    }
}
