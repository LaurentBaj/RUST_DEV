// Rust has testing built within the language
// Example of this is the assert_eq() 

#[cfg(test)]
mod tests {
    // Pass
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // Test fail
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
