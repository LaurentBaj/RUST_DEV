use std::sync::{Arc, Mutex};
use std::thread;

// A counter that is incremented to 10, each time by an independent thread
fn main() {
    // Counter needs to have many owners 
    // We can't use RC since it's not thread safe so we use ARC (atomic)
    let counter = Arc::new(Mutex::new(0)); 
    let mut handles = vec![];

    for _ in 0..10 {
        // We want to use the counter but we have already moved it
        // We can fix this by cloning counter and using it inside this scope
        // by shadowing the counter
        let counter = Arc::clone(&counter); // mutex uses interior mutability
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}