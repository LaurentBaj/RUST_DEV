#[derive(Debug)]
enum Genre {
    Rock,
    Blues,
    Jazz,
    Pop,
    Funk,
    Indie
}

struct Artist {
    genre: Genre,
    name: String,
    plays: Option<u32>
}

impl Artist {
    fn print_artist(&self) {
        println!("Artist[name: {}, genre: {:?}, plays: {}]", 
                self.name, self.genre, self.plays.unwrap_or(0));
    }
}

fn main() {
    let a1 = Artist {
        name: String::from("John Mayer"),
        genre: Genre::Rock,
        plays: Some(200000)
    };    

    a1.print_artist();
}


