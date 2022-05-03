use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Could not read string");

    let str1 = String::from("First");
    let str2 = String::from("Second");
    
    println!("\n{}\n{}", str1, str2);    
    print!("{}", input.trim().to_string());
}