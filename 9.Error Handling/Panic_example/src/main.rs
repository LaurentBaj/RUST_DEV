/*
    Panic stops the program where we expect the program to crash
    and follows the stack trace to the point of failure

    In RUST there are recoverable errors and unrecoverable ones
    Panic is an unrecoverable error since we want the program to
    stop if there's at an undedesireable behaviour 
*/


fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(22);
}

fn c(i: i32) {
    if i == 22 {
        panic!("Program panic");
    }
}

/* Error message

    thread 'main' panicked at 'Program panic', panic1.rs:15:9
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/


/*  If we use this command: $env:RUST_BACKTRACE=1; cargo run 

- This shows us that the program crashed when calling c()

        thread 'main' panicked at 'Program panic', src\main.rs:21:9
    stack backtrace:
    0: std::panicking::begin_panic_handler
                at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a\/library\std\src\panicking.rs:498
    1: core::panicking::panic_fmt
                at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a\/library\core\src\panicking.rs:116
    2: ex1::c
                at .\src\main.rs:21
    3: ex1::b
                at .\src\main.rs:16
    4: ex1::a
                at .\src\main.rs:12
    5: ex1::main
                at .\src\main.rs:8
    6: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                at /rustc/9d1b2106e23b1abd32fce1f17267604a5102f57a\library\core\src\ops\function.rs:227
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

*/