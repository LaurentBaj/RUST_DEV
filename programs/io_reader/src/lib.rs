//! # IO_PARSER
//!
//!
//! Parsing input from the `std::io` can be verbose. As beginners we do this all the time! This crate aims to simplify this process so that you can speed up the process of creating your own cli-applications.

mod io_fn {
    /// Parses user input and then returns a String
    pub mod ru {
        use std::io;

        /// Parse user input, trims it and then returns it as a String
        ///
        /// # Example
        ///
        /// ```
        /// let input = user_input();
        /// println!("{}", a); // Your input from the terminal
        ///
        /// ```
        #[allow(dead_code)]
        pub fn user_input() -> String {
            let mut input: String = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read user input");
            input
        }
    }

    /// The num module parses the user input into an integer or a float
    pub mod num {
        use crate::ru::user_input;

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

        /// Parse user input to a i32
        ///
        /// # Example
        ///
        /// ```
        /// let a: i32 = to_num();
        /// println!("{}", a); // Your input from the terminal
        ///
        /// ```
        /// Uses the `user_input()` function from the `ru` module
        #[allow(dead_code)]
        pub fn to_num() -> i32 {
            let input: String = user_input();
            parse_to_num(input)
        }

        /// Parse user input to a f32.
        ///
        /// # Example
        ///
        /// ```
        /// let a: f32 = to_num();
        /// println!("{}", a); // Your input from the terminal
        ///
        /// ```
        /// Uses the `user_input()` function from the `ru` module
        #[allow(dead_code)]
        pub fn to_float() -> f32 {
            let input: String = user_input();
            parse_to_float(input)
        }
    }
}

pub use crate::io_fn::ru;
pub use crate::io_fn::num;



