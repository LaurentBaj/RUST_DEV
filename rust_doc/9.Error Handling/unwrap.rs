use std::fs::File;


// unwrap() performs the Result-based match expression
fn main() {
    let file = File::open("hello.txt").unwrap();
    println!("{:?}", file);
}

// If file does not exist:  "The system cannot find the file specified."