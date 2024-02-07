// https://genius.com/Christmas-songs-the-twelve-days-of-christmas-lyrics

const TOTAL_DAYS: usize = 12;

const DAYS: [&str; TOTAL_DAYS] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

const LYRICS: [&str; TOTAL_DAYS] = [
    "A partridge in a pear tree",
    "Two turtle doves and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];


fn main() {
    println!("The Twelve Days of Christmas");
    println!("----------------------------");
    for day in 0..TOTAL_DAYS {
        println!();
        print_first_verse_line(day);
        print_lyrics(day);

    }
}

fn print_first_verse_line(day: usize) {
    println!(
        "On the {} day of Christmas my true love sent to me",
        DAYS[day],
    )
}

fn print_lyrics(day: usize) {
    for line in (0..=day).rev() {
        println!("{}", LYRICS[line])
    }
}
