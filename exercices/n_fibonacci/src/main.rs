use std::io::stdin;

fn main() {
    println!("Fibonacci number");
    println!("Info: you can stop the process by typing 'quit'\n");

    loop {
        println!("Please enter a number:");
        let mut input = String::new();

        let _ = stdin()
            .read_line(&mut input)
            .inspect_err(|err| eprintln!("Failed to read input {err}"));

        let input = match input.trim() {
            "quit" => break,
            other => other,
        };

        let n: u64 = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Wrong format.");
                continue;
            }
        };

        let mut x: u64 = 0;
        let mut y: u64 = 1;

        let result: Option<u64> = match n {
            0 => Some(x),
            1 => Some(y),
            _ => {
                let mut sum: Option<u64> = None;
                for _ in 1..n {
                    sum = x.checked_add(y);
                    if sum.is_none() {
                        break;
                    }
                    x = y;
                    y = sum.unwrap();
                }
                sum
            }
        };

        match result {
            Some(value) => println!("The {n} number in the Fibonacci sequence is: {value}\n"),
            None => println!("Can't compute value.\n"),
        }
    }
}
