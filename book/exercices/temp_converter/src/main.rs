// Basic implementation with data types from chapter 3.
use std::io::stdin;

fn main() {
    println!("Temperature converter");
    println!("Info: you can stop the process by typing 'quit'");

    loop {
        println!("Please enter a temperature with a one of these units (C,F), example: '22.2 C'");
        let mut input = String::new();

        let _ = stdin()
            .read_line(&mut input)
            .inspect_err(|err| eprintln!("Failed to read input {err}"));

        let input = match input.trim() {
            "quit" => break,
            other => other,
        };

        let (value, unit) = match input.split_once(" ") {
            Some(result) => result,
            None => {
                println!("Wrong format.");
                continue;
            }
        };

        let temp: f64 = match value.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Wrong format.");
                continue;
            }
        };

        match unit.to_uppercase().as_str() {
            "F" => println!("{:.1}°F = {:.1}°C", temp, to_celsius(temp)),
            "C" => println!("{:.1}°C = {:.1}°F", temp, to_fahrenheit(temp)),
            _ => {
                println!("Unknown unit.");
                continue;
            }
        }
    }
}

fn to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}

fn to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * 1.8
}
