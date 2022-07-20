use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 0..10 {
            println!("Side thread: {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    for i in 0..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/*
    Now we have to threads running independently.
    Because of how this program is written it will
    not allow the side thread to finish before program
    execution.
*/

