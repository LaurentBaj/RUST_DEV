use std::io; 

fn main() {

    // Buffers
    let mut input = String::new(); 
    let mut num = String::new();  
    let mut res: f64 = -1.0; // By default: error

    println!("Convert from Fahrenheit or Celcius (c, f)");
    
    // Read c or f
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line"); 
    
    let input = input.trim();  // Strings have white space apparently
    
    println!("Enter °{}: ", input.to_uppercase());
    
    // Read number
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");    
    
    // Convert user input (num) to int    
    let num: f64 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => -1.0,
    };

    if input == "c" { 
        let res = celcius_to_fahrenheit(num);
        println!("{}°F", res); 
    } else if input == "f" {
        let res = fahrenheit_to_celcius(num);
        println!("{}°C", res); 
    } else {
        println!("Did not recognize input"); 
    }
}


fn fahrenheit_to_celcius(fah: f64) -> f64 {
    (fah - 32 as f64) / 1.8 
}

fn celcius_to_fahrenheit(cel: f64) -> f64 {
    (cel * 1.8) + 32 as f64 
}