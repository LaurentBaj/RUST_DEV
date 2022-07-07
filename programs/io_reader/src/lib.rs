/// IO-PARSER
///
/// As a a beginner it can be daunting to rewrite functions that parses inputs from the terminal.
///
/// This is typical as a beginner, since you're using the '''std::io''' library when working with inputs
///
/// This library will parse user inputs, trim them and then return a String


mod io_fn {
    pub mod read_input {
        use std::io;

        #[allow(dead_code)]
        pub fn user_input() -> String {
            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read user input");
            input
        }
    }

    pub mod numerize {
        use crate::read_input::user_input;

        #[allow(dead_code)]
        fn parse_to_num(input: String) -> i32 {
            match input.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => -1,
            }
        }

        #[allow(dead_code)]
        fn parse_to_float(input: String) -> f32 {
            match input.trim().parse::<f32>() {
                Ok(num) => num,
                Err(_) => -1.0,
            }
        }

        #[allow(dead_code)]
        pub fn read_num() -> i32 {
            let input: String = user_input();
            parse_to_num(input)
        }

        #[allow(dead_code)]
        pub fn read_float() -> f32 {
            let input: String = user_input();
            parse_to_float(input)
        }
    }
}

pub use crate::io_fn::read_input;
pub use crate::io_fn::numerize;



