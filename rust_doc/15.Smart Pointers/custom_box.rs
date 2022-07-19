// Struct tuple
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/*
    fn main() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

   --

    We have created a box that should behave the same way as a Box, but does not
    when we run the code above. This is because our struct does not
    implement the deref trait such as 'Box' does
*/

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T; // Associate type (Chapter. 19)

    fn deref(&self) -> &Self::Target {
        &self.0 // Get the first value inside struct (There is only one val)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // Now MyBox implements the deref trait so we can dereference it!
}