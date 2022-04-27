use std::io; 

fn main() {
    println!("CREATE A NEW USER");
    
    let username = save_field("username".to_string()); 
    let password = save_field("password".to_string()); 
    compare(&password);

    println!("User created [ {} -- password: {} ]", 
        username, password);
}


fn save_field(text: String) -> String {
    println!("Enter new {}: ", text);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
    // input.trim(); -- This turns str into a pointer so we need to toString it
}


fn compare(prev: &String) {
    println!("Confirm password");

    loop {
        let mut comp_password = String::new();
        io::stdin()
            .read_line(&mut comp_password)
            .expect("Failed to read line");
        let comp_password = comp_password.trim().to_string();

        if prev.to_string() == comp_password {
            break;
        } else {
            println!("Passwords did not match. Try again: "); 
        }
    }
}

