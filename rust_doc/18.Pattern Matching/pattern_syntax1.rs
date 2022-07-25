//// Matching literals: Output is "one"
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}


//// Named Variables
// y will be shadowed since it is redeclared inside the match
// y will match anything it is compared to, such as x in this case
// Output: Matched, y = 5\n at the end: x = Some(5), y = 10
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {:?}", x, y);


// Matching with the OR operator
let x = 1;

match x {
    1 | 2 => println!("one or two"), // This is the result 
    3 => println!("three"),
    _ => println!("anything"),
}


// Matching with ranged values: ..=
let x = 5;

match x {
    1..=5 => println!("one through five"), // res
    _ => println!("something else"),
}

let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"), // res
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
