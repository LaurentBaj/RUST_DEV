use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("Hi!"),
            String::from("How are you?"),
            String::from("I'am fine! Thank you!"),
            String::from("What are you up to?"),
            String::from("Nothing much, just scrolling 9gag"),
            String::from("That is sexy!")
        ];


        for i in messages {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });

    // rx is a responder: a data structure of messages
    // Iterate over it and display each message
    for i in rx {
        println!("Got: {}", i);
    }

    /* - This is ok for a single message 
    let recieved = rx.recv().unwrap(); -- recv blocks the main thread execution
    println!("Got: {}", recieved); */
}