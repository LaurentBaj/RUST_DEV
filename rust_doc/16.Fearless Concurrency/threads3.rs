use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // take ownership of v with 'move'
    let handle = thread::spawn(move || {
        // v is no longer valid outside this block
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

/*
    Rust's strict ownership rules ensures that we write quality code.
    In this case, Rust does not allows us to pass v as a ref into a 
    new thread since it does not now how long it will last because it 
    might create a dangling reference. Therefore we move the ownership 
    into the closure.
*/