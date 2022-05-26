#[derive(Debug)]
struct Location<T> {
    latitude: T,
    longitude: T,
}

impl<T> Location<T> {
    fn new(latitude: T, longitude: T) -> Location<T> {
        Location { latitude, longitude }
    }

    fn latitude(&self) -> &T { &self.latitude }
}

fn main() {
    let p1 = Location::new(5.11, 4.22);
    let p2 = Location::new("Laurent", "Jon Olav");

    println!("{:?}", p1);

    println!("{:?}", p2);
    println!("{:?}", p2.latitude()); // "Laurent"
}


/*
    Location { latitude: 5.11, longitude: 4.22 }
    Location { latitude: "Laurent", longitude: "Jon Olav" }
*/