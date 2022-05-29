struct Book {
    title: String,
    author: String,
    release_date: u32
}

struct Magazine {
    title: String,
    release_date: u32
}

impl Summary for Book {
    fn summarize(&self){
        println!("Book [author: {}, title: {}, published: {}]", 
                self.author, self.title, self.release_date)
    }
}

impl Summary for Magazine {
    fn summarize(&self) {
        println!("Magazine [title: {}, published: {}]", 
                self.title, self.release_date)
    }
}

// Like interfaces in Java - Rust does not need method body
pub trait Summary { 
    fn summarize(&self);

    /* Default implementation 
    fn summary(&self) -> String {
        format!("No item specified in summary")
    } 
    */
}


fn main() {
    let book = Book { 
        title: String::from("Women"),
        author: String::from("Charles Bukowski"),
        release_date: 1989
    };

    book.summarize();
    // Book [author: Charles Bukowski, title: Women, published: 1989]

    let magazine = Magazine {
        title: String::from("Nuts and Bolts"),
        release_date: 2020
    };

    print_summary(&magazine); 
    // Magazine [title: Nuts and Bolts, published: 2020]
}

// We can create a function that only works on all types that impl Summary
fn print_summary(litterature: &impl Summary) {
    litterature.summarize();
}