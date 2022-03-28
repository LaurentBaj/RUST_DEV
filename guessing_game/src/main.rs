use rand::Rng;
use std::cmp::Ordering;
use std::io;


// println! and stdin() are macro functions
// MFs are function within the language

fn main() {
    println!("Guess the number!");

    // Generate random rumber
    let secret_number = rand::thread_rng().gen_range(1..101);
	
    loop {
        println!("Please input your guess.");

        // Create buffer for user input
	    // Since it's stdin it needs to be a String
        let mut guess = String::new();

	    // Read user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

	    // Match is like if-else
	    // trim - remove white spaces
	    // parse - string -> int
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

	    // {} is like % in C when printing
        println!("You guessed: {}", guess);

	    // Ordering is a lib that allows cmp() -> compare fn
	    // Ordering comes also with enum: Less, Greater and Equal
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

