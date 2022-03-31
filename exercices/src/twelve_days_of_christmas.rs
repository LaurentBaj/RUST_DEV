fn main() {
 
    let verses = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh","eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];


    for i in 0..days.len() {
        println!("\nOn the {} day of christmas my true love sent to me:", days[i]);
 
        let x = i + 1;
        for j in (0..x).rev() {
            println!("{}", verses[j]); 
        }
    }
}