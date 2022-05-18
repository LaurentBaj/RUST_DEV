#[derive(Debug)]
enum Department {
    Sales,
    Engineering,
    Admin,
    Hr,
    Undefined
}

use std::collections::HashMap;
fn main() {
    let mut v = Vec::new();
    v.push(add_entry());
    v.push(add_entry());
    v.push(add_entry());

    println!("{:?}", v);
}

fn add_entry() -> HashMap<String, Department> {
    let mut map = HashMap::new();
    
    println!("Enter name");
    let name = user_input();
    println!("Enter department");
    let department = enter_dep(user_input().as_str());

    map.insert(name, department);
    map
}


fn user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    input.trim().to_string()
}

fn enter_dep(str: &str) -> Department {
   match str {
       "Sales" => { Department::Sales }
       "Engineering" => { Department::Engineering }
       "Admin" => { Department::Admin }
       "Hr" => { Department::Hr }
       _=> { Department::Undefined }
    } 
}

/* fn add_to_list(list: &mut Vec<HashMap<String, Department>>) {
    list.push(add_entry());
} */