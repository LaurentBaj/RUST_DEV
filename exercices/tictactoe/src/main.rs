use std::io;

fn main() {
    println!("\nWelcome to Tic Tac Toe!\n");

    let mut slots = create_slots();
    print_board(&mut slots);
    let someoneHasWon = false;

    // Make sure it's every other player turn
    for i in 0..9 {
        let turn = match i % 2 { 
            0 => "O", 
            _ => "X" 
        };
        mark_slot(turn, &mut slots);
        
        if check_winner(turn, &mut slots) { 
            println!("'{}' is the winner!", turn);
            break; 
        }
    }
    
    if someoneHasWon {
        println!("GAME OVER");
    }
}

//// Read player input and mark a slot
fn mark_slot(player: &str, slots: &mut [String; 9]) {
    while true {            
        println!("'{}'s turn\n", player);
        let input = user_input();    
        let input_num = match input.parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                println!("Input must me an integer");
                continue;
            }
        };

        if input_num < 1 || input_num > 9 {
            println!("Number was not in range");
            continue;
        }

        let current = &slots[input_num - 1];
        
        if current == "O" || current == "X" {
            println!("Slot has already been taken");            
        } else {
            slots[input_num - 1] = String::from(player);
            print_board(slots);
            break;
        }
    } 
}

fn check_winner(player: &str, slots: &mut [String; 9]) -> bool {
    let player = String::from(player);

    if slots[0] == player && slots[1] == player && slots[2] == player { // Horizontal
        true 
    } else if slots[4] == player && slots[5] == player && slots[6] == player {
        true
    } else if slots[7] == player && slots[8] == player && slots[9] == player {
        true
    } else if slots[1] == player && slots[4] == player && slots[7] == player { // Horizontal
        true
    } else if slots[2] == player && slots[5] == player && slots[8] == player {
        true
    } else if slots[3] == player && slots[6] == player && slots[9] == player {
        true
    } else if slots[1] == player && slots[5] == player && slots[9] == player { // Diagonal
        true
    } else if slots[7] == player && slots[5] == player && slots[3] == player {
        true
    } else {
        false
    }
}

// Read input, trim it and convert it to a string
fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read input");
    input.trim().to_string()
}

fn print_board(array: &mut [String; 9]) {
    print!("  \n      |     |        ");
    println!("\n    {} |  {}  | {}     ", array[6], array[7], array[8]);
    println!("  - - + - - + - -  ");
    println!("    {} |  {}  | {}       ", array[3], array[4], array[5]);
    println!("  - - + - - + - -  ");
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







