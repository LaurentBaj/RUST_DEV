pub fn adder(number1: i32, number2: i32) -> i32 {
    number1 + number2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() { 
        assert_eq!(4, adder(2, 2));
        println!("Result: {}", adder(2, 2));
        // To show print: cargo test -- --show-output 
    }
}
