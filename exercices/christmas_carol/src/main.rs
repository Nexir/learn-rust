const SENTENCES: [[&str; 2]; 12] = [
    ["first", "A partridge in a pear tree"],
    ["second", "Two turtle doves and"],
    ["third", "Three french hens"],
    ["fourth", "Four calling birds"],
    ["fith", "Five golden rings"],
    ["sixth", "Six geese a-laying"],
    ["seventh", "Seven swans a-swimming"],
    ["eigth", "Eight maids a-milking"],
    ["ninth", "Nine ladies dancing"],
    ["tenth", "Ten lords a-leaping"],
    ["eleventh", "Eleven pipers piping"],
    ["twelth", "Twelve drummers drumming"],
];

fn main() {
    println!("The Twelve Days of Christmas\n");

    for day_nb in 0..SENTENCES.len() {
        print_verse_start(SENTENCES[day_nb][0]);
        for i in (0..=day_nb).rev() {
            println!("{}", SENTENCES[i][1]);
        }
        println!();
    }
}

fn print_verse_start(day: &str) {
    println!("On the {day} day of Christmas, my true love sent to me");
}
