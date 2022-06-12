use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let file = File::open("hello.txt");

    // If file does not exists: create it
    let file = match file {
        Ok(file) => file,
        Err(err) => match err.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("Error: {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
            // We need to supply ErrorKind with an alternative action if the 
            // above conditions are not met
        }
    };    
}

/*
    In this example, we use match arms to deal with multiple conditions.
    However, this is verbose and can be handled more developer friendly
    - Check file_example3.rs
*/