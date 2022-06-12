// In RUST does not have any null values
// This is where the Option<T> comes in
// It deals with null safety and is integrated in RUST

/*
    enum Option<T> {
        Some(T),
        None,
*/


fn main() {

    // Rust can inferr the type with Some
    let x = Some(5);
    let y = Some("Hello");

    // Null has to be annotated with type
    let absent_value: Option<u32> = None;

    let z = 5;

    // let res = z + x; this does not work since they are different types
    // x has to be unwrapped
    let res = z + x.unwrap_or(0); // Default 0 - reffer to value passed previously
    println!("{}", res); // 10

    println!("{}", z + absent_value.unwrap_or(0)); // 5
}