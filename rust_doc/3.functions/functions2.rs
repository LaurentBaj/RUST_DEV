fn main() {
    let y = {
        let x = 3; // statements
        x + 1 // expression
    };

    println!("The value of y is: {}", y);
}

/*
Note that the x + 1 line doesn’t have a semicolon at the end, unlike most of the lines you’ve seen so far. 
Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn 
it into a statement, and it will then not return a value. Keep this in mind as you explore function return 
values and expressions next. 
*/