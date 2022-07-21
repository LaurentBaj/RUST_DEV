use io_parser::ru;
mod lib; 
use lib::chat_utils::{ funs, model };

fn main() {
    println!("Enter some input");
    let p1 = model::Person::new("Laurent");

    let res = funs::chat_input(ru::user_input(), &p1);
    println!("{}", res);
}







