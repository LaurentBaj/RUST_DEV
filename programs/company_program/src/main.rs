use std::io;
use std::fmt::Debug;


enum Company {
    Accenture,
    Bouvet,
    Capgemini,
    Sopra_Steria
}


struct Employee<T: Debug> {
    name: Option<T>,
    /* company: Option<Company>,
    age: Option<T> */
}



impl<T: Debug> Employee<T> {
    fn new_employee ( name: Option<T> ) -> Employee<T> {
        Employee { name }
    }
    
    fn print_emp(&self) {
        match &self.name {
            Some(name) => println!("Employee name: {:?}", name),
            None => println!("Error printing employee name")
        }
    }
}


fn main() {
    println!("Enter employee name");
    let emp = Employee::new_employee(Some(read()));
    emp.print_emp(); // Emplyee name: "Karoline"
}


fn read() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading");
    input.trim().to_string()
}

