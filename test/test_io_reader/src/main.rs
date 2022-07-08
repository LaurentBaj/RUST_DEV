use io_reader::{ru, num};

fn main() {
    println!("Enter input: ");
    let a: String = ru::user_input();
    println!("\n{}\n", a);

    println!("Enter number: ");
    let a: i32 = num::to_num();
    println!("\n{}\n", a);

    println!("Enter float: ");
    let a: f32 = num::to_float();
    println!("\n{}\n", a);
}
