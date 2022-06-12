// Vectors are resizable arrays
// The size is not needed at compile time

fn main () {
    // let numbers: Vec<i32> = Vec::new();
    // Vector-type can be inferered 
    // VECs do not implement the copy trait
    // so we pass the ref to the loop
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    for n in &numbers {
        println!("{}", n);
    }

    numbers.push(9);
    numbers.push(10);
    numbers.push(11);
    numbers.pop() // Remove last element 

    let third = &numbers[2];
    println!("{}", third); // 3

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    // * is dereferencing in Rust
    // We have to add mut in the loop even if vec is already mutable
}