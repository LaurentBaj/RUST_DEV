use std::io;

fn main() {
    println!("Enter number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Entered wrong value"); 

    let input: u32 =  match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };
        
    let res = fibonacci(input);
    println!("Result = {}", res); 
}

fn fibonacci(x: u32) -> u32 {
    if x == 0 || x == 1 {
        return 1;
    } 

    fibonacci(x-1) + fibonacci(x-2)
}