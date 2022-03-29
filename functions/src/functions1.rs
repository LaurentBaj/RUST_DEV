fn main() {
    println!("Hello, world!");

    another_function(); // Hello, world!

    addition(366, 677); // 1043
}

fn another_function() {
    println!("Another function.");
}

fn addition(num1: i32, num2: i32) {
    println!("{}", num1 + num2); 
}
