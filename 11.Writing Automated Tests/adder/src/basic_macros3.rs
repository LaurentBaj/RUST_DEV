pub fn add_two(a: i32) -> i32 {
    a + 2
}

// The assert_eq!() and assert_ne!() need to two params
// We could use ass
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn fail() {
        assert_ne!(3, add_two(2));
        assert_ne!(5, add_two(2));
    }
}