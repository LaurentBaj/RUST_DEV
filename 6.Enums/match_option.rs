fn main() {
    let x = Some(5); // 5
    let y = increment(x); // 6
    let none = increment(None); // None
}

fn increment(num: Option<i8>) -> Option<i8> {
    match num {
        None => None,
        Some(num) => Some(num + 1) 
    }
}

// Option<T> needs to use the None type for null safety
