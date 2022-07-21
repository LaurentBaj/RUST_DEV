use std::thread;
use std::sync::{mpsc};
use std::time::Duration;
use crate::lib::{Message, Person};

pub fn chat(input: String, p: &Person) -> String {
    let (tx, rx) = mpsc::channel();
    let author = p.name.clone();
    let msg = Message::new(input.as_str(), &p);

    tx.send(msg.content).unwrap();
    thread::sleep(Duration::from_secs(2));
    let res = rx.recv().unwrap();

    format!("{}: '{}'", author, res)
}

