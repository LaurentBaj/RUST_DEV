use std::io;

fn main() {
    println!("Enter an integer:");
    let a = user_input();

    let res = ui_to_num(a);
    println!("Your integer: {:?}", res.unwrap());
}


fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error when reading user input");
    input.trim().to_string()
}

fn ui_to_num(input: String) -> Result<i32, io::ErrorKind> {
    match input.parse::<i32>() {
        Ok(n) => Ok(n),
        Err(e) => panic!("Error: {:?}", e)
    }
}
