#[allow(dead_code)]
fn calculate(x: i32, y: i32, op: fn(i32, i32) -> i32) -> i32 {
    op(x, y)
}

fn main() {
    // Closures
    let add = |x: i32, y: i32| x + y;
    let sub = |x: i32, y: i32| x - y;
    let mul = |x: i32, y: i32| x * y;
    let div = |x: i32, y: i32| x / y;

    // calculate() takes in two i32 and a fn which in this case is anonymous
    assert_eq!(calculate(2, 2, add), 4);
    assert_eq!(calculate(8, 4, sub), 4);
    assert_eq!(calculate(-2, -2, mul), 4);
    assert_eq!(calculate(16, 4, div), 4);
}

