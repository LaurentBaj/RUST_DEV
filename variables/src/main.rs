use std::io;

fn main() {

    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; - convention for const

    // Variables are mutable by default
    // mut allows me to create mutable var
    println!("Enter a number");
    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line"); 
    
    // Shadowing -> Rust allows me to redeclare variables
    // This uses the same space, unlike let which delcares new space in memory        
    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    

    println!("Print your number: {}", num);
}
