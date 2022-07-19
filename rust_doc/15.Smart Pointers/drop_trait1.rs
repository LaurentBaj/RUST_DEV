// The drop trait allows for better code cleanup

// In this example we show how we can define som behaviour on a struct when it is dropped
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

/* OUTPUT:
    ---
    CustomSmartPointers created.
    Dropping CustomSmartPointer with data `other stuff`!
    Dropping CustomSmartPointer with data `my stuff`!"
    ---
    Firstly, c and d are created. Then the print msg is called.
    Lastly, when main is finished, c and d goes out of scope and
    the code we defined (when implementing the drop trait) is executed.
    Notice how 'd' is dropped before 'c' since it follows the stack
*/