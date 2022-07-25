fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(n) = stack.pop() {
        println!("{}", n);
    }
}

/* OUTPUT: 
    3
    2
    1
*/

