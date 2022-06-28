fn main() {

    // Num example
    let num_vec: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let nv_iter = num_vec.iter();

    for i in nv_iter {
        println!("{}", i);
    }

    println!("{}\n", num_vec[2]); // 3

    
    // Vec string example
    let string_vec: Vec<String> = vec![
        String::from("Ben"),
        String::from("Arzana"),
        String::from("Osman"),
        String::from("Magbule"),
        String::from("Trym")
    ];    

    let sv_iter = string_vec.iter();
    for s in sv_iter {
        println!("{}", s);
    }
}