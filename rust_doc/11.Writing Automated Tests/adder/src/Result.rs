#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(()) // pass
        } else {
            Err(String::from("two plus two does not equal four")) // fail
        }
    }
}

// Instead of using Rust macros, we can use the Result Enum instead