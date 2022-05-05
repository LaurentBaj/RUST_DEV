fn main() {
    let str1 = String::from("Pathologic");
    let str2 = take_and_return_str(str1);
    let sp = &str2;

    // println!("{}\n{}", str2, sp); -- prints Pathologic twice
    
    let mut str3 = String::from("Endless Space 2");
    let mut sp2 = &str3;
    let mut sp3 = &str3;
    change(&mut str3); 
    println!("{}", str3);  
}

fn take_and_return_str(input: String) -> String {
    input
}

fn change(input: &mut String){
    input.clear();
    input.push_str("Inside");
}

