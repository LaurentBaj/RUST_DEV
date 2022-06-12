pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// Customizing error messages is handy in a specific case such as this
// assert! can either take in one or two arguments
// one: conditional requirement - two: conditional requirement + custom error message
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("Ben")); -> Test would panic and we would not know why
        assert!( result.contains("Ben"), "Names did not match" ); 
    }
}

/*
    ---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Names did not match', src\lib.rs:14:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

*/