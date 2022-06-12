// Lifetimes prevents danling references

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// lifetimes are defined with tics following a name - 'a
// lifetime annotations are generics
// The return lifetime is defined by the param with she shortest lifetime
// In main: the lifetime of string1 and string2 are the same
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 


/*  

fn longest(x: & str, y: & str) -> & str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 

^ expected named lifetime parameter
|help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter

*/


// &i32 - a reference
// &'a i32  - a reference with an explicit lifetime
// &'a mut i32 - a mutable reference with an explicit lifetime