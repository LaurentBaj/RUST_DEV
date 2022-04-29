use std::io; 

fn main() {
    println!("CREATE A NEW USER");
    
    let username = create_username(); 
    let password = create_password(); 

    println!("User created [ {} -- password: {} ]", 
        username, password);
}

fn create_username() -> String {
    println!("Enter new username: ");
    user_input()
}

fn create_password() -> String {
    println!("Enter new password: ");
    let input = user_input();
    compare(&input);
    input
}

fn compare(prev: &String) {
    println!("Confirm password");

    loop {
        let comp_password = user_input();

        if prev.to_string() == comp_password {
            break;
        } else {
            println!("Passwords did not match. Try again: "); 
        }
    }
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Input could not be read");
    input.trim().to_string()   
}

// input.trim(); -- This turns str into a pointer so we need to toString it


