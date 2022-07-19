enum List {
    Cons(i32, Box<List>),
    Nil,
}

/* This is not allowed since Box 'steals' values and does not impl the copy trait:
    fn main() {
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a)); -- a is moved to b
        let c = Cons(4, Box::new(a)); -- a is empty and c tries to use it (ERROR)
    }
*/

// RC - Reference Counting: is a smart pointer that makes it easier for a single val
// to have multiple references. This is done by counting the amount of references a val
// currently holds. If the count is empty then the value can be cleaned up without any
// dank pointers.

