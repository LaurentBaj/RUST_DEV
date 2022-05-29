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
    
    fn print_emp(&self) -> String {
        match &self.name {
            Some(name) => format!("Emplyee name: {:?}", name),
            None => format!("Error printing employee")
        }
    }
}


fn main() {
    println!("Enter employee name");
    let emp = Employee::new_employee(Some(read()));
    println!("{}", emp.print_emp());
}


fn read() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading");
    input.trim().to_string()
}

