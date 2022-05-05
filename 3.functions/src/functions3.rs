fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

// Arrow indicates that it is a return fn
fn plus_one(x: i32) -> i32 {
    x + 1
}