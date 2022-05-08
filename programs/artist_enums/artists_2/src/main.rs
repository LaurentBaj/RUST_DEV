#[derive(Debug)]
enum Software {
    app,
    website
}

#[derive(Debug)]
enum Developer {
    software(Software)
}

struct Employee {
    name: String,
    dev: Developer
}

impl Employee {
    fn print_dev(&self) {
        println!("Employee [ Name: {} - Title: {:?}-developer]", 
                self.name, self.dev)
    }
}

fn main() {
    let developer = Employee {
        name: String::from("Laurent"),
        dev: Developer::software(Software::app)
    };

    developer.print_dev();
}



