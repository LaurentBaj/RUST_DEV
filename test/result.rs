#[derive(Debug)]
struct Phone {
    number: String
}

impl Phone {
    fn new(number: String) -> Phone {
        Phone { number: number.trim().to_string() }
    }
}

fn main() {
    println!("{:?}", reg_phone().unwrap());
}

fn enter_number() -> Phone {
    println!("Enter phone number");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Error");
    Phone::new(number)
}

fn reg_phone() -> std::io::Result<Phone> {
    Ok(enter_number())
}

