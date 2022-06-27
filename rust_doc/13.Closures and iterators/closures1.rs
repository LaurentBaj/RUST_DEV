#[allow(unused_variables)]

fn main() {
    let x = 21;
    let get_answer = |y| x + y;
    println!("{}", get_answer(x)); // 42

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    let f = add(21, 21); //  Function pointer

    // Adder with functions
    let f = |x: i32, y: i32| { x + y };

    // Simplified closure because of single expression
    let f = |x: i32, y: i32| x + y ;

    // Closure with inferred parameter types
    let f = |x, y| x + y;


    println!("{}", f(2, 2));
}

