// front_of_house does not need to be pub for eat_at_restaurant() since they are siblings
// in the same module. "hosting" is, by default private, which is why we need to add pub.
// With this we can access the path to the add_to_waitlist(), but not the function itself
// for the same reasons as hosting, so we add pub to that one as well. 

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}


// Super lets ut refer to parent ressources
// fix_incorrect_order() can refer to serve_order()
// since it lies in the parent structure
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}