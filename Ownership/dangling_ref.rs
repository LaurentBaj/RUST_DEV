// This does not work since the value of 
// 'str' is popped of when fn-dangle is 
// finished executing

fn main() {
    let sp: &String = dangle();
}

fn dangle() -> &String {
    let str = String::from("This gets popped");
    &str
}