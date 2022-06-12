mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;
// use self::front_of_house::hosting; for relative path

// We can also do this for functions, but not for data types
// use crate::front_of_house::hosting::add_to_waitlist;


pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}