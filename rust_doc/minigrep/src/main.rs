use std::fs;
use std::env;

// cargo run the poem.txt
fn main() {
    let args: Vec<String> = env::args().collect();

    let content = Config::new(&args).filename;
    let outputs = fs::read_to_string(content.as_str())
        .expect("Could not read poem.txt");

    println!("With text:\n{}
    ", outputs);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Config { query, filename }
    }
}