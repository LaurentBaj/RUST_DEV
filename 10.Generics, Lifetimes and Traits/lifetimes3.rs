use std::fmt::Display;

fn main() {
    longest_with_an_announcement("abcd", "xyz", 42); // Announcement! 42
}

// Notice how we have to specify the lifetime in the fn with the Generic
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

