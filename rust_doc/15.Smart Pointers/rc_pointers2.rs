enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    println!("Pointers to a: {}", Rc::strong_count(&a)); // Count: 3

    {
        let d = Cons(4, Rc::clone(&a));
        println!("Count: {}", Rc::strong_count(&a)); // Count: 4
    }

    // 'd' goes out of scope so reference count is decremented
    println!("Pointers to a: {}", Rc::strong_count(&a)); // Count: 3
}