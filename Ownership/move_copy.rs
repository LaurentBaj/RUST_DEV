fn main() {
    // Primitives can be copied this way
    let a = 2;
    let b = a; // b -> 2

    // Strings are 'moved' this way
    // Value gets carried over
    let str1 = String::from("Hello");
    let str2 = str1; // This is a move
    
    // We can copy value of a String like this
    let str3 = str2.clone(); 

    println!("{}", b); // 2
    // println!("{}", str1);  -- str1 does not exist anymore
    println!("{}", str2); // hello
    println!("{}", str3); // hello 
}