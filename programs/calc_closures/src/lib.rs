#[allow(dead_code)]


fn calculate(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addtion() {
        let x = calculate(2, 2, |x, y| x + y);
        assert_eq!(x, 4);
    }

    #[test]
    fn test_subtraction() {
        let x = calculate(2, 2, |x, y| x - y);
        assert_eq!(x, 0);
    }

    #[test]
    fn test_multiplication() {
        let x = calculate(2, 2, |x, y| x * y);
        assert_eq!(x, 4);
    }

    #[test]
    fn test_division() {
        let x = calculate(2, 2, |x, y| x / y);
        assert_eq!(x, 1);
    }
}
