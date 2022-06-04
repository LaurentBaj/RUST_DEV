use std::io;


fn read_input() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Error");
    buf.trim().to_string()
}

fn parser(str: String) -> Result<i32, String> {
    match str.parse() {
        Ok(n) => Ok(n),
        Err(_) => Err(String::from("Error"))
    }
}

fn main() {
    let s = read_input();
    let t = parser(s);
    println!("{:?}", t);
}

/*
    - Ok(42) 
    - Err("Error")
*/