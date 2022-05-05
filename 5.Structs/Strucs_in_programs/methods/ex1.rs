#[derive(Debug)]

struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    fn calc_area(&self) -> u32 {
        self.length * self.width
    }

    fn is_square(&self) -> bool {
        if self.length == self.width { true } else { false }
    }

    // Do not need to pass two params for this since self is implied
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width
    }

    // Getter
    fn width(&self) -> u32 {
        self.width
    }
}

/*  - Method could also be impl somewhere else -
    impl Rectangle {
        fn is_square(&self) -> bool {
        if self.length == self.width { true } else { false }
        }
    } 
*/

fn main() {
    let rect1 = Rectangle {
        length: 5,
        width: 3
    };

    let rect2 = Rectangle { 
        length: 2,
        width: 2
    };

    println!("{}", rect1.calc_area()); // 15
    println!("{}", rect1.is_square()); // false
    println!("{}", rect1.can_hold(&rect2)); // true
    println!("{}", rect1.width()); // 3

    // Lib above allows this format
    println!("{:?}", rect1); // Rectangle { length: 5, width: 3 }
}