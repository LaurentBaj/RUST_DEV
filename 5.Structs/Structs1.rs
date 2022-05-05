struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    let user1 = User {
       email: String::from("even_august@gmail.com"),
       username: String::from("EvenAug"),
       active: true,
       sign_in_count: 3
    };

    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.active);
    println!("{}", user1.sign_in_count);
    /*
        EvenAug
        even_august@gmail.com
        true
        3
    */

    let user2 = User {
        username: String::from("PederAm"),
        ..user1
    };
    println!("\n{}", user2.username);
    println!("{}", user2.email);
    println!("{}", user2.active);
    println!("{}", user2.sign_in_count);

    // user1.email does not exist since we have passed its string
    // to user2. user1.active and user1.sic are primitive values
    // that use the internal copy trait
    
    // println!("{}", user1.email); - error

}

