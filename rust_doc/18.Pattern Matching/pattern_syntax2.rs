/* Destructings */

//// Structs
// Ex1:
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p;
assert_eq!(0, a);
assert_eq!(7, b);

// Ex2: 
let Point { x, y } = a;
assert_eq!(0, x);
assert_eq!(7, y);


//// Matching with destructuring 
let p = Point { x: 0, y: 7 };

// 1. arm: if y is 0 then match no matter what x is
// 2. arm: same but for x
// 3. arm: match anything (same as a tuple initialization)
// The second arm will match and ignore the third 
match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y), 
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}


//// ENUMS
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

// Notice how we use curly brackets for structs and tuples
// Quit does not contain any values to destructure
match msg {
    Message::Quit => {
        println!("The Quit variant has no data to destructure.")
    }
    Message::Move { x, y } => {
        println!(
            "Move in the x direction {} and in the y direction {}",
            x, y
        );
    }
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!(
        "Change the color to red {}, green {}, and blue {}",
        r, g, b
    ),
}


//// Nested Data Structs
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
        "Change the color to red {}, green {}, and blue {}",
        r, g, b
    ),
    Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
        "Change the color to hue {}, saturation {}, and value {}",
        h, s, v
    ),
    _ => (),
}

//// Advanced destructuring
struct Point { x: i32, y: i32 }

let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
// feet: 3, inches: 10 || x: 3, y: -10


