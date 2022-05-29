// Traits are like 'interfaces'. They define shared behaviour

struct Guitar {
    name: String,
    is_tuned: bool
}

struct Piano {
    name: String,
    is_tuned: bool
}

pub trait Playable {
    fn summarize(&self) -> String;
}

impl Playable for Piano {
    fn summarize(&self) -> String {
        format!("Piano name: {}, is_tuned: {}", 
                self.name, self.is_tuned)
    }
}

impl Playable for Guitar {
    fn summarize(&self) -> String {
        format!("Guitar name: {}, is_tuned: {}", 
                self.name, self.is_tuned)
    }
}

fn main() {
    let p = Piano {
        name: String::from("Yamaha"),
        is_tuned: true
    };

    let g = Guitar {
        name: String::from("Fender"),
        is_tuned: false
    };

    println!("{}", p.summarize());
    println!("{}", g.summarize());
}