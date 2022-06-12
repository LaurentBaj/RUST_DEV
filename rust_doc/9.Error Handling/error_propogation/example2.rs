use std::fs::File;
use std::io;
use std::io::Read;

// Less verbose version of ex1
// ? - does the same thing as unwrap() so we won't need Resut Match expressions
// ? - only works in a fn where we're return a Return enum
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s) // Returns Result type with file contents (as string)
}

