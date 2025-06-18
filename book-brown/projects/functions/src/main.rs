fn main() {
    println!("Hello, world!");

    another_function();
    another_function_1(10);
    print_label_measurements(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn another_function_1(x: i32) {
    println!("The value of x is: {x}");
}

fn print_label_measurements(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
