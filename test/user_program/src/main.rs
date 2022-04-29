mod fields;


fn main() {
    println!("CREATE A NEW USER");
    
    let username = fields::create_username(); 
    let password = fields::create_password(); 

    println!("User created [ {} -- password: {} ]", 
        username, password);
}
