// Constant, can never be mutable (no mut keyword permited) and must specify type.
// '_' symbol to allow this unsued constant
// naming convention: SNAKE_CASE_IN_UPPERCASE
// evaluated at compile time to 10_800
const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Immutable variable
    let immutable = 5;
    println!("The value of immtable is: {immutable}");
    // Mutable variable
    let mut mutable = 5;
    println!("The value of mutable is: {mutable}");
    mutable = 6;
    println!("The value of mutable is: {mutable}");

    // Initial variable x that will be shadowed
    let x = 5;
    // Shadowing
    // mut != let (shadowing creates a new variable, can change the type of the variable and reuse the same name and it's still immutable)
    let x = x + 1; // 6

    {
        // Block scope
        // This x variable is only defined in the scope of this block
        // Shadowing
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
        // Inner shadowing ends for this scope
    }

    println!("The value of x is: {x}"); // 6

    let spaces = "   "; // Type "string"
    let spaces = spaces.len(); // Type "number"
    // insted of having spaces_str and spaces_num we can keep the same name

    let mut spaces = "   ";
    // spaces = spaces.len(); // doesn't compile 'mismatched types'
}
