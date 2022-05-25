use std::fs;

fn main() {
    println!("{:?}", read_file()); 
}

fn read_file() -> String {
    let content = fs::read_to_string("helo.txt");
    content.unwrap().trim().to_string()
}

// "Hello there, young man!"