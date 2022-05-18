#[derive(Debug)]
struct Person {
    name: String,
    age: Option<u8>
}

fn main() {
    let p = create_person();
    println!("\nPerson [Name: {:?} -- Age: {:?}]\n", 
            p.name, p.age.unwrap());
}


fn create_person() -> Person {
    let mut p1 = Person { name: String::from(""), age: Some(0)};
    let mut _age: String = String::new();
    let mut _name: String = String::new();
    
    println!("Enter name: ");
    std::io::stdin()
        .read_line(&mut _name) 
        .expect("Could not read input");
    
    println!("Enter age: ");
    std::io::stdin()
        .read_line(&mut _age)
        .expect("Could not read input");

    let _age = match _age.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let _name = _name.trim().to_string();

    p1.age = Some(_age);
    p1.name = String::from(_name); 

    p1
}