// derive is an attribute
// Derive macro generating an impl of the trait Debug.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // All functions defined in impl block are called 'associated functions'

    // Type 'Self' is an alias
    // self is short gor 'self: &Self'
    // Method definition with self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Multiple impl block for the same Struct is valid syntax
impl Rectangle {
    // Constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 10;
    let height1 = 20;
    println!(
        "The area of the rectangle is {} square pixels.",
        area_int(width1, height1)
    );

    let tuple = (30, 40);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(tuple)
    );

    // Instanciate struct
    let rect = Rectangle {
        width: 50,
        height: 60,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    // Debug
    println!("rect is {:?}", rect);
    // Formatted Debug
    println!("rect is {:#?}", rect);

    let scale = 2;
    let rect2 = Rectangle {
        // Prints and returns the value of a given expression for quick and dirty debugging.
        // The macro works by using the Debug implementation of the type of the given expression to print the value to stderr
        // along with the source location of the macro invocation as well as the source code of the expression
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);

    if rect2.width() {
        println!(
            "The area of the rectangle is {} square pixels.",
            // Method call
            rect2.area()
        );
    }

    /*
    Info no '->' operator, Rust has autmatic deferencing
    object->something() same as (*object).something()
    In rust:
    p1.distance(&p2) = (&p1).distance(&p2)
     */

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    // :: syntax used for assocaited functions and namespaces created by modules
    let square = Rectangle::square(10);
    println!("square {:#?}", square);
}

fn area_int(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
