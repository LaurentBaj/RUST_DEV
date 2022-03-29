use std::io; 

fn main() {
    let mut input = String::new(); 
    let mut num = String::new();  
    let mut res: f64 = 0.0;

    println!("Convert from Fahrenheit or Celcius (c, f)");

    // Read c or f
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); 
    
    let input = input.trim();  
    println!("Enter a number: ");

    // Read number
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");    
    
    let num = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    if input == "c" { 
        println!("Enter cel: ");
        res = celcius_to_fahrenheit(num as f64)
    } else if input == "f" {
        println!("Enter fah: ");
        res = fahrenheit_to_celcius(num as f64)
    }

    println!("{}", res); 
}


fn fahrenheit_to_celcius(fah: f64) -> f64 {
    (fah - 32 as f64) / 1.8 
}

fn celcius_to_fahrenheit(cel: f64) -> f64 {
    (cel * 1.8) + 32 as f64 
}

