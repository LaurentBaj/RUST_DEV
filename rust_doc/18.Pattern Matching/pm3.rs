fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

/** OUTPUT:
    a is at index 0
    b is at index 1
    c is at index 2

    We use the enumerate method to adapt an iterator to produce 
    a value and that value’s index in the iterator, placed into 
    a tuple. The first value produced is the tuple (0, 'a'). 
    When this value is matched to the pattern (index, value), 
    index will be 0 and value will be 'a', printing the first line of the output."
*/