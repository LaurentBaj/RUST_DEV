struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

/*
    fn main() {
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        c.drop(); -- !! Won't work since rustc would drop it twice: here and after block
        println!("CustomSmartPointer dropped before the end of main.");
    }

*/


use std::mem::drop;

fn main() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c); // c is dropped before block is finished and the drop-fn is ex before last println
    println!("CustomSmartPointer dropped before the end of main.");
}


/* OUTPUT:
    ---
    CustomSmartPointer created.
    Dropping CustomSmartPointer with data `some data`!
    CustomSmartPointer dropped before the end of main.
    ---

    We can free memory by ourself. This is particularly useful if we want to lock a value
*/