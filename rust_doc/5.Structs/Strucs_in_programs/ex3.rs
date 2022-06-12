struct Rectangle {
    width: u32,
    length: u32
}

fn calc_area(rec: &Rectangle) -> u32 {
    rec.width * rec.length
}

fn main() {
    let rec1 = Rectangle {
        length: 5,
        width: 3
    };
    println!("{}", calc_area(&rec1));
}