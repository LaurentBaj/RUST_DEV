pub fn affirm_args(input: Vec<String>) {
    if input.len() < 2 {
        panic!("To few arguments");
    } else {
        let mut numbers = into_num_vec(input);
        bubble_sort(&mut numbers);
        print_numbers(numbers);
    }
}


fn into_num_vec(input: Vec<String>) -> Vec<i32> {
    let mut collection: Vec<i32> = Vec::new();
    
    for i in input {
        match i.trim().parse() {
            Ok(n) => collection.push(n),
            Err(_) => panic!("Error parsing")
        }
    }

    collection
}

fn print_numbers(input: Vec<i32>) {
    for i in input {
        println!("{}", i);
    }
}

fn bubble_sort(input: &mut Vec<i32>) {
    for _i in 0..input.len() {
        for j in 0..input.len() - 1 {
            if input[j] > input[j + 1] {
                input.swap(j, j +1);
            }
        }
    }
}