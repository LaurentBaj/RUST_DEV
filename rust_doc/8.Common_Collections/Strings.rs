// Strings in depth
// Strings are internally Vec<u8>
// They are represented as numbers in the UTF-8

fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3); // format does not take ownership like println 
    /* println!("{}-{}-{}-\n{}-", s1, s2,s3, s); 
        tic-tac-toe-
        tic-tac-toe- 
    */

    let a = add("Hello, world!");
    for x in a.chars() { // Hello, world!
        print!("{}", x);
    }
    for x in a.bytes() { // Byte representation
        println!("{}", x);
    }
}

// We pass a str which is a String slice, or a String literal
fn add(s: &str) -> &str {
   s
}