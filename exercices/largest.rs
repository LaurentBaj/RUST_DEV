struct Largest {
    index: i32,
    value: i32
}

fn main() {
    let array = [2, 9, 10, 333, 400, 4, 1, 7, 227];
    
    // Tuple version
    println!("\nLargest index: {}, value: {}\n", 
            largest_i32_tuple(&array).0, 
            largest_i32_tuple(&array).1
    );

    // Struct version
    println!("\nLargest index: {}, value: {}\n",
            largest_i32_struct(&array).index,
            largest_i32_struct(&array).value
    );

    // Rustlang version -> get largest value
    println!("\nLargest value: {}\n", largest_value(&array));
}

fn largest_i32_tuple(slice: &[i32]) -> (i32, i32) {
    let mut largest = 0;

    for i in 0..slice.len() {
        if slice[i] > slice[largest] {
            largest = i;
        }
    }

    (largest as i32, slice[largest]) // largest needs to be coerced: usize -> i32
}

fn largest_i32_struct(slice: &[i32]) -> Largest {
    let mut largest = 0;

    for i in 0..slice.len() {
        if slice[i] > slice[largest] {
            largest = i;
        }
    }

    Largest { index: largest as i32, value: slice[largest] }
}

fn largest_value(slice: &[i32]) -> i32 {
    let mut largest = slice[0];

    for &item in slice {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/* Output: 
    
    Largest index: 4, value: 400


    Largest index: 4, value: 400


    Largest value: 400

*/