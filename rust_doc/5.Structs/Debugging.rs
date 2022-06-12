#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale), // Width is 60, but is also debugged
        height: 50,
    };

    dbg!(&rect1);
}