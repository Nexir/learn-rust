mod front_of_house;

// use crate::front_of_house::hosting;
// caller: resturant::front_of_house::hosting::add_to_waitlist()
use crate::front_of_house::hosting::add_to_waitlist;

// Re-exposing
pub use crate::front_of_house::hosting;
// caller: resturant::hosting::add_to_waitlist()

pub fn eat_at_restaurant() {
    // Absolut path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path
    self::front_of_house::hosting::add_to_waitlist();

    // After use keyword, the hosting module is accesible in the scope
    // Idiomatic way to bring a function in scope
    hosting::add_to_waitlist();
    // Not idiomatic
    add_to_waitlist();

    // Order a breakfast in the simmer with Rye toast.
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like.
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not
    // allowed to see or modify the seasonal fruit comes with the meal.
    // meal.seasonal_fruit = String::from

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // All variants are public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // Relative path with super, access parent
        super::deliver_order();
    }

    fn cook_order() {}
}

// Use parents modules to avoid conflicts
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    todo!()
}

fn function2() -> io::Result<()> {
    todo!()
}

// Use alias to avoid conflict
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    todo!()
}

fn function4() -> IoResult<()> {
    todo!()
}
