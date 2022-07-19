// Pointers in Rust

// Most common type is a reference. These contain the address of a value which they're pointing to.
// References only borrow the values which they point to

// Smart Pointers are data types with different capabilities such as owning the data that they're
// pointing to: Vec<T>, String

fn main() {
    // Dereference
    let x = 5;
    let y = &x; // y is not a value but an address

    assert_eq!(5, x);
    assert_eq!(5, *y); // * - dereferences y to get its value


    // Smart Pointer: Box
    let x = 5;
    let y = Box::new(x); // Point to a copied value of x (does not take ownership of x in this case)

    assert_eq!(5, x);
    assert_eq!(5, *y); // Dereference such as above
}


