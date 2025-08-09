//Prints the 12 days of xmas

fn main() {
    let days: [&str; 12] = 
    [   "first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eight",
        "ninth", "tenth", "eleventh", "twelfth"];

    let gifts: [&str; 12] = [
        "A partridge in a pear tree!",
        "Two turtle doves, and",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for i in 0..12 {
        let day = days[i];
        println!("\nOn the {day} day of Christmas, my true love gave to me");
        
        for k in (0..i+1).rev() {
            let gift = gifts[k];
            println!("{gift}");
        }

    }
}
