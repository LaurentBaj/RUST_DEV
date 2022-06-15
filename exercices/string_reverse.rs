fn main() {
    println!("{}", reverse("Hello")); // olleH
}

pub fn reverse(input: &str) -> String {
    let mut res = String::new();
    for i in input.chars().rev() {
        res.push(i);
    }
    res
}