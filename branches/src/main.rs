fn main() {
    let number = 6;

    // branches are called arms
    if number < 5 {
        println!("condition is true"); // arm
    } else {
        println!("conditon is false"); // arm
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // match first
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4,3, or 2");
    }

    let condition = true;
    // if is an expression
    let number = if condition {
        // block evaluate to last expression
        5 // numbers are by themselves exepssions
    } else {
        6 // all arms results types must be the same
    };
    println!("The value of number is: {number}");
}
