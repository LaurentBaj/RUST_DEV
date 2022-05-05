#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    age: u32
}

// Custom print
impl User {
    fn print(&self) {
        println!("USER [ username: {} -- email: {} -- active: {} -- age {} ]", 
            self.username, self.email, self.active, self.age);
    }
}

fn main() {
    let user1 = create_user();

    // USER [ username: Laurent -- email: laurent@hotmail.com -- active: true -- age 26 ]
    user1.print(); 
    
    /* -- Standard compound print --
        println!("{:?}", user1);
        User { username: "laurent", email: "laurent@gmail.no", active: true, age: 26 }
    */
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    input.trim().to_string()
}

fn create_user() -> User {
    let mut user = User {
        username: String::new(),
        email: String::new(),
        active: false,
        age: 0
    };

    println!("Enter username:");
    user.username = get_input();

    println!("Enter email:");
    user.email = get_input();

    println!("Is your user active? (y or n)");
    if get_input() == "y" {
        user.active = true
    }

    println!("Enter age:");
    user.age = match get_input().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    user
}


