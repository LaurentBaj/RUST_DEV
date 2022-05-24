use std::fs;
use std::io;

// The least verbose version of all the other examples
// Note that we have to import the entire 'fs' module
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// fs::read_to_string() -> Result<String>