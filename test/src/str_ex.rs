use std::io; 

fn main() {
    println!("CREATE A NEW USER");
    println!("Enter username: ");
    let username = save_field(); 

    println!("Enter new password: ");
    let password = save_field(); 

    println!("User[ {} -- password: {} -- valid: {}]", 
        username, password, compare(&password));
}


fn save_field() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
    // input.trim(); -- This turns str into a pointer so we need to toString it
}

fn compare(prev: &String) -> bool {
    println!("Confirm password");
    let mut new = String::new();
    io::stdin()
        .read_line(&mut new)
        .expect("Failed to read line");
    let new = new.trim().to_string();

    if prev.to_string() == new {
        true
    } else {
        false
    }
}

