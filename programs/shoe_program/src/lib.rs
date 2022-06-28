#[derive(PartialEq, Debug)]
struct Shoe {
    name: String,
    size: u8
}

impl Shoe {
    #[allow(dead_code)]
    fn new(name: String, size: u8) -> Shoe {
        Shoe { 
            name, 
            size
        }
    }
}

#[allow(dead_code)]
fn shoes_that_fit_me(shoes: Vec<Shoe>, shoe_size: u8) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tes_shoe_that_fit_me() {
        let shoes: Vec<Shoe> = vec![
            Shoe::new(String::from("Nike"), 43),
            Shoe::new(String::from("Adidas"), 42),
            Shoe::new(String::from("Umbro"), 43)
        ];

        let for_me = shoes_that_fit_me(shoes, 43);

        assert_eq!(
            for_me,
            vec![
                Shoe::new(String::from("Nike"), 43),
                Shoe::new(String::from("Umbro"), 43)
            ]
        )
    }
}
