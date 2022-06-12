use std::fs::File;


// .expect() does the same as unwrap() but now we can specify the error message
fn main() {
    let file = File::open("hello.txt").expect("Failed to open 'hello.txt'");
}

// If file does not exist:  'Failed to open 'hello.txt'