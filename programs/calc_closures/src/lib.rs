#[allow(dead_code)]
fn calculate(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}

#[allow(dead_code)]
fn add(x: i32, y: i32) -> i32 { x + y }

#[allow(dead_code)]
fn sub(x: i32, y: i32) -> i32 { x - y }

#[allow(dead_code)]
fn mult(x: i32, y:i32) -> i32 { x * y }

#[allow(dead_code)]
fn div(x: i32, y: i32) -> i32 { x / y }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addtion() {
        let res = calculate(2, 2, add);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_subtraction() {
        let res = calculate(2, 2, sub);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_multiplication() {
        let res = calculate(21, 21, mult);
        assert_eq!(res, 441);
    }

    #[test]
    fn test_division() {
        let res = calculate(81, 9, div);
        assert_eq!(res, 9);
    }
}
