
/*
    Here we want to change the value of a string by using its reference.
    When we declare the variable we have specify that it is a mutable variable.
    We also have to define mutability when we define fn-arguments 
*/

fn main() {
    let mut s = String::from("Hello");
    change(&mut s); 
    println!("{}", s); // 's' exists 
}


fn change(str: &mut String) {
    str.push_str(", World!"); 
}
