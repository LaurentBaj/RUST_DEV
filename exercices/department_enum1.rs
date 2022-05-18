#[derive(Debug)]
enum Department {
    Sales,
    Engineering,
    Admin,
    Hr,
    Undefined
}

fn main() {
    let input1 = user_input();
    let dep = match_option(&input1); 
    println!("{:?}", dep);
}

fn user_input() -> String {
    println!("Enter department"); 
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    input.trim().to_string()
}

fn match_option(string: &String) -> Department {
   match string.as_str() {
       "Sales" => { Department::Sales }
       "Engineering" => { Department::Engineering }
       "Admin" => { Department::Admin }
       "Hr" => { Department::Hr }
       _=> { Department::Undefined }
    } 
}