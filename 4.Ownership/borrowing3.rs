
fn main() {
    // Since we are not mutating 's' then it can have many references
    let s = String::from("This works!");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}, {}", s, r1, r2);

    /* 
    This creates a race condition:
    We would have to mut pointers
    That can alter a value
    - let mut s = String::from("Line 15 would'nt work");
    - let mut r1: &mut String = &mut s;
    - println!("{}, {}", s, r1);
    */

    

    // A reference exists from where it's declared
    // to the last place it's used  
    let mut s = String::from("Works");
    let r1: &String = &s;
    let r2: &String = &s; 
    // let r3: &mut String = &mut s; --  We cannot have mut and imut pointers
    println!("{}, {}, {}", s, r1, r2); // r1, r2 are dropped since it's their last usage

    let r3: &mut String = &mut s;
    println!("{}", r3);
}