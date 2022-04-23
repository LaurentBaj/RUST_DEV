
fn main() {
    // Since we are not mutating 's' then it can have many references
    let mut s = String::from("This ");

    let sp: &mut String = &mut s;
    sp.push_str(" works!");
    println!("{}", sp); 
}