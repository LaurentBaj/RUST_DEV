use std::io;

fn main() {
    println!("Enter number");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error");

    // Entering a char, String or float will give -1
    let guess = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => -1
    };

    println!("Your number: {}", guess);
}