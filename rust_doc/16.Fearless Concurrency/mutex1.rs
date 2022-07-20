use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    { 
        let mut num = m.lock().unwrap(); // lock aquires a lock -> Result
        *num = 6;
    } // Lock will be released after it goes out of scope automatically

    println!("m = {:?}", m);
}