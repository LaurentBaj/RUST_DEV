fn main() {
    fizz_buzz(100);
}


fn fizz_buzz(input: u32) {
    for x in 1..=100 {
       game_logic(x);
    }
}

fn game_logic(input: u32) {
    if input % 3 == 0 && input % 5 == 0 {
        println!("FizzBuzz");
    } else if input % 3 == 0 {
        println!("Fizz");
    } else if input % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", input);
    }
}
