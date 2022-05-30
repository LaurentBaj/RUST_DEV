use std::fmt::Display;

fn main() {
    let mut numbers = [2, 10, 32, 7, 9, 5];
    let mut floats = ["d", "c", "b", "a"];
    bubble_sort(&mut numbers);
    bubble_sort(&mut floats);
}

pub fn bubble_sort<T: Ord + Display>(arr: &mut [T]) {
    for _i in 0..arr.len() {
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    print_array(&arr);
}

fn print_array<T: Display>(arr: &[T]) {
    println!("");
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
}



