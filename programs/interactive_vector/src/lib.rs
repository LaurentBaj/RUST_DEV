use std::io;

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    input.trim().to_string()
}

fn get_integer() -> i32 {
    match read_input().parse::<i32>() {
        Ok(n) => n,
        Err(_) => panic!("Error parsing integer")
    }
}

fn print_numbers(list: &Vec<i32>) {
    for i in 0..list.len() {
        if i == 0 {
            print!("\nYour list: [ {}, ", list[i]);
        } else if i == list.len() - 1 {
            print!("{} ]", list[i])
        } else {
            print!("{}, ", i);
        }
    }
}

pub fn program(list: &mut Vec<i32>) {
    println!("\nNUMBER LIST");

    loop {
        println!("\nDo you wish to add to list? (y, n)");
        let input = read_input();

        if input == "y" {
            println!("Enter integer: ");
            list.push(get_integer());
        } else if input == "n" {
            print_numbers(&list);
            break;
        } else {
            println!("Invalid input. Try again (y, n)");
        }
    }
}


