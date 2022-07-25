/** Ignoring Values in a Pattern */

// Ignoring parameters
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

foo(3, 4);


// Ignoring collection items
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
}

//// Ignoring rest of the values inside a collection
// Ex1:
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}

// Ex2: First and last element
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {}, {}", first, last);
    }
}
