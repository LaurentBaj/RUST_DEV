use std::fs::File;

fn main() {
    // File::open() returns a io::Result<File>
    let file = File::open("hello.txt");

    // File might not exist in dir so we need to deal with it 
    let file = match file {
        Ok(file) => file,
        Err(err) => panic!("Error: {}", err)
    };    
}

// 'Error: The system cannot find the file specified.
// - We get this error since hello.txt does not exist in this path