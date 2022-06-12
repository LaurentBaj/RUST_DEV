use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T
}

impl<T: Display + PartialOrd> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    fn compare_members(&self) {
        if self.x < self.y {
            println!("Member x is smaller than y");
        } else if self.x > self.y {
            println!("Member x is larger than y");
        } else {
            println!("Members are equal");
        }
    }
}

fn main() {
    let p1 = Point::new(2, 4);
    let p2 = Point::new(4, 4);
    let p3 = Point::new(5, 4);
    let p4 = Point::new(false, true);
    /* Using different types
    let p2 = Point::new(2.5, 4.5);
    let p3 = Point::new("Karoline", "Nielsen");
    let p4 = Point::new(true, false); 
    */

    p1.compare_members();
    p2.compare_members();
    p3.compare_members();
    p4.compare_members();

    /*
        Member x is smaller than y
        Members are equal
        Member x is larger than y
        Member x is larger than y
    */
}