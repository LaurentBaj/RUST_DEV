//mod print; 
mod math;

fn main() {

    // print::run() - Hello World!

    let x = math::add(2, 2);
    let y = math::subtract(10, 6);
    let z = math::multiply(2, 2);
    let o = math::divide(16.8, 4.2); // Has to be f32

    println!("{}", x); 
    println!("{}", y); 
    println!("{}", z);
    println!("{}", o);
}
