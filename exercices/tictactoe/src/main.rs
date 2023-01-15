use std::io;

fn main() {
    let mut slots = create_slots();
    println!("\nWelcome to Tic Tac Toe!\n");
    print_board(&mut slots);

    // Make sure every other player turn
    for i in 0..9 {
        let turn = match i % 2 { 0 => "O", _ => "X" };
        mark_slot(turn, &mut slots);
    }
}

//// Read player input and mark a slot
fn mark_slot(player: &str, slots: &mut [String; 9]) {
    println!("'{}'s turn\n", player);

    // Read input, trim it and convert it to a string
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    let input = input.trim().to_string();
    
    // Mark slot with 'X' or 'O'
    let input_num = input.parse::<usize>().unwrap() - 1;
    slots[input_num] = String::from(player);
    print_board(slots);
}

fn print_board(array: &mut [String; 9]) {
    print!("  \n      |     |        ");
    println!("\n    {} |  {}  | {}     ", array[6], array[7], array[8]);
    println!("   - -| - - | - -  ");
    println!("    {} |  {}  | {}       ", array[3], array[4], array[5]);
    println!("   - -| - - | - -  ");
    println!("    {} |  {}  | {}     ", array[0], array[1], array[2]);
    println!("      |     |      \n");
}

//// Create board
fn create_slots() -> [String; 9] {
    let slots = [
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
        String::from("5"),
        String::from("6"),
        String::from("7"),
        String::from("8"),
        String::from("9")
    ];

    slots
}







