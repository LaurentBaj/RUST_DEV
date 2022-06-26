fn main() {
    let e1: Item = Item::Weapon {
        name: String::from("Iron Blade"),
        damage: 67,
        range: Range::Short
    };

    println!("{:?}", e1);


    let e2 = Item::Mineral(String::from("Cobolt"));
    println!("{:?}", e2);
}

#[derive(Debug, PartialEq)]
enum Range {
    Short,
    Medium,
    Long
}

#[derive(Debug, PartialEq)]
enum Item {
    Weapon { name: String, damage: u8, range: Range},
    Equipment { name: String, resistance: u8 },
    Mineral(String)
}



/* impl Enum {
    fn new_weapon(name: &str, damage: u8, range: Range) -> Item::Weapon {
        let name = String::from(name);
        let name = String::from(name);
        let range = 
        Item::Weapon { name, damage, range }
    }
} */



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        let e1: Item = Item::Weapon {
            name: String::from("Iron Blade"),
            damage: 67,
            range: Range::Short
        };
 
        assert_eq!(
            Item::Weapon {
                name: String::from("Iron Blade"),
                damage: 67,
                range: Range::Short
        },  e1);

        let e2 = Item::Mineral(String::from("Cobolt"));
        assert_eq!(Item::Mineral(String::from("Cobolt")), e2);
    }

}

