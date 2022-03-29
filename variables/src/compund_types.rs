// Compound types can group multiple values into one type. 
// Rust has two primitive compound types: tuples and arrays.

fn main() {

    /* TWO PRIMARY COMPOUND TYPES: Tuples and Arrays

    A tuple is a general way of grouping together a number of values with a variety of types 
    into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size

    
    Unlike a tuple, every element of an array must have the same type. 
    Unlike arrays in some other languages, arrays in Rust have a fixed length
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring 
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    // Ways of accessing them
    println!("The value of y is: {}", y);

    // Or like this..
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    
    // ARRAYS
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    
    
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Love this!
    let a = [3; 5]; // 3 3 3 3 3

    // Accessing array contents works the same as in other languages
    println!("{}", a[4]); // 3            
}