use std::thread;
use std::time::Duration;

fn main() {
    let complete_this_thread = thread::spawn(|| {
        for i in 0..10 {
            println!("Side thread: {}", i);
            thread::sleep(Duration::from_millis(2));
        }
    });

    complete_this_thread.join().unwrap();
    println!();

    for i in 0..5 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

/* OUTPUT:
    ---
    Side thread: 0
    Side thread: 1
    Side thread: 2
    Side thread: 3
    Side thread: 4
    Side thread: 5
    Side thread: 6
    Side thread: 7
    Side thread: 8
    Side thread: 9

    Main thread: 0
    Main thread: 1
    Main thread: 2
    Main thread: 3
    Main thread: 4
    ---

    - Join halts the main thread
    - Join allows us to run theese threads concurrently.
    - Join can only be called from a stored thread such as above.
*/
