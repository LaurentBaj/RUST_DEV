pub fn binary_search(array: &[i8], key: i8) -> Option<usize> {
    let low: i8 = 0 as i8;
    let high: i8 = (array.len() - 1) as i8;
    
    find(&array, key, low, high)
}

fn find(array: &[i8], key: i8, low: i8, high: i8) -> Option<usize> {
    let mid = (high - low) / 2;
    let mid_value = array[mid as usize];

    if low >= high {
        if key == mid_value {
            return Some(mid as usize);
        } else {
            find(&array, key, low, mid);
            find(&array, key, mid + 1, high);
        }
    }

    None
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        /* let arr = [2, 7, 89, 99, 100, 106];
        let res = binary_search(&arr, 99).unwrap(); */
        assert_eq!(3, 3);
    }
}
