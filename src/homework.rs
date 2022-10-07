// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

// *** HOMEWORK 1 ***
// Fahrenheit 32°F freezing 212°F boiling (for water of course)
// Celcius 0°C freezing 100°C boiling
const FDELTA: f64 = 212.00 - 32.00;
const CDELTA: f64 = 100.00;
const RATIO: f64  = FDELTA / CDELTA;
// println!("ratio: {}", ratio);

pub fn convert_from_fahr_to_celc(fahr: i32) -> f64 {
    let f: f64 =  fahr.into(); // fahrenheit
    // println!("Fahrenheit: {}", f); // debug
    let result: f64 = (f - 32.00) / RATIO;
    return result;
}

pub fn convert_from_celc_to_fahr(celc: i32) -> f64 {
    let c: f64 =  celc.into(); // fahrenheit
    // println!("Fahrenheit: {}", f); // debug
    let result: f64 = (c * RATIO) + 32.00;
    return result;
}

// *** HOMEWORK 2 ***
// Fibonacci number
pub fn fibn(number: u64) -> u64 {
    if number == 0 || number == 1 {
        return 1;
    } else {
        return fibn(number - 2) + fibn(number - 1);
    }
}

// *** HOMEWORK 3 ***
// The Twelve Days of Christmas lyrics
// *****************************
// On the first day of Christmas
// My true love sent to me
// A partridge in a pear tree!
//
// Two turtle doves
// On the second day of Christmas
// My true love sent to me
// Two turtle doves
// And a partridge in a pear tree!
//
// Three french hens
// On the third day of Christmas
// My true love sent to me
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Four calling birds
// On the fourth day of Christmas (what's a calling bird)
// My true love sent to me
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Five golden rings!
// On the fifth day of Christmas
// My true love sent to me
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Six geese a layin'
// On the sixth day of Christmas
// My true love sent to me
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Seven swans a swimmin'
// On the seventh day of Christmas
// My true love sent to me
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Eight maids a milkin'
// On the eighth day of Christmas
// My true love sent to me
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds (calling birds)
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Nine ladies dancin'
// On the ninth day of christmas
// My true love sent to me
// Nine ladies dancin'
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Ten lords a leapin'
// On the tenth day of Christmas
// My true love sent to me
// Ten lords a leapin'
// Nine ladies dancin'
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Eleven pipers pipin'
// On the eleventh day of Christmas
// My true love sent to me
// Eleven pipers pipin'
// Ten lords a leapin'
// Nine ladies dancin'
// Eight maids a milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!
//
// Twelve drummers drummin'
// On the twelfth day of Christmas
// My true love sent to me
// Twelve drummers drummin'
// Eleven pipers pipin'
// Ten lords a leapin'
// Nine ladies dancin'
// Eight maids milkin'
// Seven swans a swimmin'
// Six geese a layin'
// Five golden rings!
// Four calling birds
// Three french hens
// Two turtle doves
// And a partridge in a pear tree!

pub fn lyrics() {
    let verse0 = "On the";
    let verse1 = "day of Christmas";
    let verse2 = "My true love sent to me";
    let verse3 = "A partridge in a pear tree!";
    let order = [
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
        "twelfth"
    ];
    let verses = [
        "Twelve drummers drummin'",
        "Eleven pipers pipin'",
        "Ten lords a leapin'",
        "Nine ladies dancin'",
        "Eight maids milkin'",
        "Seven swans a swimmin'",
        "Six geese a layin'",
        "Five golden rings!",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves"
    ];
    let mut verses2 = verses;
    verses2.reverse();
    let len = 12;
    for i in 1..len + 1 {
        if i == 1 {
            println!("{} {} {}", verse0, order[i - 1], verse1);
            println!("{}", verse2);
            println!("{}", verse3);
        } else {
            println!("{}", verses2[i - 2]);
            println!("{} {} {}", verse0, order[i - 1], verse1);
            println!("{}", verse2);
            print_cum_arr(&verses2, len - 1, i - 2);
            println!("{}", verse3);
        }
        println!("");
    }
}

fn print_cum_arr(arr: &[&str], len: usize, i: usize) {
    let mut counter = 0;
    while counter <= i {
        println!("{}", arr[i - counter]);
        counter += 1;
    }
}
