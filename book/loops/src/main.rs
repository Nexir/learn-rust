fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // the result can be specified after break
            break counter * 2;
        }
    }; // semicolon for the let statement

    println!("The result is {result}");

    // Multiples loops within loops, break and continue apply to the innermost loop at that point.
    // Or use loop label
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // breaks the loop with label counting_up
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFOFF!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value is: {element}");
    }

    // (1..4) range from 1 to 4 excluded: 1, 2, 3
    // .rev() reverse it: 3,2,1
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!")
}
