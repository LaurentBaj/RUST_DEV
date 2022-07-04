use sort_program::*;

use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    affirm_args(args); 
}
