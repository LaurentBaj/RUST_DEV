fn main(){
    let word = String::from("Hello World!");

    // let str1 = &word[0..5]; - cumbersome
    // let str2 = &word[6..12]; - cumbersome
    let str1 = &word[..5]; // Beginning -> index 5
    let str2 = &word[6..]; // Index 6 -> end of string
    let str3 = &word[..]; // Stores entire string

    // str1 = Hello, str2 = World!, str3 = Hello World!
    println!("str1 = {}, str2 = {}, str3 = {}", str1, str2, str3);

    let array = [0, 1, 2, 3, 4, 5];
    let a1 = &array[..2];
    let a2 = &array[3..];
    
    // 0, 1, 2, 3, 4, 5,
    print_numbers(&a1);
    print_numbers(&a2);
}

fn print_numbers(arr: &[i32]) {
    let size = arr.iter().size
    for i in arr {
        print!("{}, ", i);
    }
}
