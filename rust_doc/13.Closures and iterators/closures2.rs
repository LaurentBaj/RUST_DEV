#[allow(unused_variables)]

fn main() {
    // Passing function pointers
    calc_and_print(2, 2, add); // regular
    calc_and_print(2, 2, |x, y| x + y); // anonymous/closure
}

fn calc_and_print(num1: i32, num2: i32, cal: fn(i32, i32) -> i32) {
    let res = cal(num1, num2);
    println!("{}", res);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}