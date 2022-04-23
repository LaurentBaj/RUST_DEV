/*
    We can move ownership in different ways.
    This is, however, tedious.
*/ 

fn main() {
    let string1 = gives_ownership();
    let string2 = String::from("Hello World!"); // Is dropped
    let string3 = take_own_and_give_back(string2); 

    println!("{} \n{}", string1, string3); 
}

fn take_own_and_give_back(str: String) -> String {
    let res = String::from(str);
    res 
}


fn gives_ownership() -> String {
    let str =  String::from("Somewhere");
    str
}