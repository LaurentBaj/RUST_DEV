mod chat_ops;
mod lib;

use lib::Person;
use io_parser::ru;

fn main() {
    println!("Enter some input");
    let p1 = Person::new("Laurent");
    chat_ops::chat(ru::user_input(), &p1);
}







