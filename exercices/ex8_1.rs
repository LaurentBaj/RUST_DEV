use std::collections::HashMap;

fn main () {
    let mut int_list = vec![1, 9, 1, 4, 4, 4, 4, 12, 5, 9, 3, 2, 18, 6, 7, 5, 5];
    int_list.sort();

    let median = int_list[int_list.len() / 2]; 
    println!("Median: {}", median);

    let mut map = HashMap::new();
    for i in int_list {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut most_occuring_value = 0;
    for (key, value) in &map {
        if value == map.values().max().unwrap() {
            most_occuring_value = *key;
            break;
        }
    }

    println!("{:?}", most_occuring_value);
}