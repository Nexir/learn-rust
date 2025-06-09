fn main() {
    another_function(5, 'h');

    let y = 6; // Statement
    // let x = (let y = 6) assignment is not an expression
    let y = {
        //expression that is a block that evaluates to 4
        let x = 3; // let statement
        x + 1 // no semicolon, returns value 
    };

    println!("The value of y is: {y}");
    let x = five();
    println!("The return value of the function is: {x}");

    let x = plus_one(5);
    println!("The result value of the plus_one(5) = {x}")
}

// Parameters types must be declared
fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

// i32 specifies returned type
fn five() -> i32 {
    5 // no semicolon, expression -> return
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon, expression -> return
}

/*
fn plus_one_statement(x: i32) -> i32 {
    x + 1; // semicolon becomes a statement -> does not compile, conflict with return type
    //Error: mismatched types expected `i32`, found `()`
}
 */
