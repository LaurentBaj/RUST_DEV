// Program that output a format of a person's full name 
// Sindre Sauarlia -> "Sa.Sindre"


#[derive(Debug)]
struct Person {
    surename: String,
    lastname: String
}

impl Person {
    fn format(&self) -> String {
        let sn = String::from(&self.surename);
        let ln = String::from(&self.lastname);
        format!("{}{}.{}", 
            &ln[0..1], &ln[ln.len() - 1..], sn)
    }
}

fn main() {
    let p1 = create_person();
    println!("{:?}", p1.format());
}

fn read_input(input: &str) -> String {
    println!("Enter {}: ", input);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Error 1");
    input.trim().to_string()
}

fn create_person() -> Person {
    println!("Create person\n");
    let person = Person {
        surename: read_input("surename"),
        lastname: read_input("lastname")
    };
    person
}