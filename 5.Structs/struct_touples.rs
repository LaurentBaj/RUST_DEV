// We can also make something called Struct touples
struct RGB(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = RGB(0, 0, 0);
    let origin = Point(0, 0, 0);
}