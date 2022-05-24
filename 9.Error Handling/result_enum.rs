// Result is a built in Rust-enum that allows use to deal with erros 
// without stoping the program: recoverable type

fn main() {
    enum Result<T, E> {
        Ok(T),
        Err(E)
    }
}