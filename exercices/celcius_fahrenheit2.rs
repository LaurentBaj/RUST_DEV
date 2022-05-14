use std::io; 

fn main() {
    println!("Convert from Fahrenheit or Celcius (c, f)");
    
    let input = read_input(); // Read c or f
    println!("Enter °{}: ", input.to_uppercase());
    let num = read_input().parse::<f32>().unwrap(); // Read input + convert to f32

    match input.as_str() {
        "c" => { println!("{}°F", celcius_to_fahrenheit(num)); },
        "f" => { println!("{}°C", fahrenheit_to_celcius(num)); },
        _=> { println!("Did not recognize input"); }
    }
}

fn fahrenheit_to_celcius(fah: f32) -> f32 {
    (fah - 32 as f32) / 1.8 
}

fn celcius_to_fahrenheit(cel: f32) -> f32 {
    (cel * 1.8) + 32 as f32 
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    input.trim().to_string()
}