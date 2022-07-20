fn main() {
    let mut array = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut vector = vec![1, 2, 3, 4, 5, 6, 7, 8];

    swap(&mut array, 0, 7);
    swap(&mut vector, 0, 7);

    assert_eq!(array[0], 8);
    assert_eq!(array[7], 1);
    assert_eq!(vector[0], 8);
    assert_eq!(vector[7], 1);
}

fn swap<T: Copy>(array: &mut [T], a: usize, b: usize) {
    let temp = array[a];
    array[a] = array[b];
    array[b] = temp;
}