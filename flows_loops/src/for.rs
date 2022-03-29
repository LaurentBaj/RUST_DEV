fn main() {
    let a = [10, 20, 30, 40, 50];

    // Traditional foreach
    for element in a {
        println!("the value is: {}", element);
    }

    println!("\n\n");

    for number in 1..4 {
        println!("{}", number);
    }


    println!("\n\n");
    
    for number in (1..4).rev() {
        println!("{}", number);
    }
}