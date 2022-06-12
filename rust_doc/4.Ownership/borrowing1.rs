// We don't want to pass ownership from 's' since it will be dropped
// from main scope, but we can pass its reference if we want to find
// something about it, such as length. 

fn main() {
    let s = String::from("Hello, World!");
    println!("{}", calc_length(&s)); // 13
}


fn calc_length(s: &String) -> usize {
    let res = s.len();
    res
}


// We cannot mutate the value of String such as it is now
// Borrowing2.rs will cover this