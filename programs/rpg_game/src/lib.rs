#[derive(Debug, PartialEq)]
pub enum Class {
    Mage,
    Swordsman,
    Depraved,
    Archer,
    Thief,
    None
}

impl Class {
    pub fn add_class(str: &str) -> Class {
        match str {
            "Mage" => Class::Mage,
            "Swordsman" => Class::Swordsman,
            "Depraved" => Class::Depraved,
            "Archer" => Class::Archer,
            "Thief" => Class::Thief,
            _=> panic!("Class does not exist")
        }
    }
}

pub trait Entity {
    fn speech(&self) -> String;
}

#[derive(Debug, PartialEq)]
pub struct Player {
    name: String,
    class: Class
}

impl Player {
    pub fn new(name: &str, class: &str) -> Player {
        Player { 
            name: String::from(name),
            class: Class::add_class(class)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Enemy {
    name: String,
    class: Class
}

impl Enemy {
    pub fn new(name: &str, class: &str) -> Enemy {
        Enemy { 
            name: String::from(name),
            class: Class::add_class(class)
        }
    }
}


impl Entity for Player {
    fn speech(&self) -> String {
        let mut preamble = String::from("My name is ");
        preamble.push_str(&self.name);
        preamble
    }
}

impl Entity for Enemy {
    fn speech(&self) -> String {
        let mut preamble = String::from("Beware! For I am ");
        preamble.push_str(&self.name);
        preamble
    }
}


pub fn world_character(entity: &impl Entity) -> String {
    entity.speech()
}


// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_creation() -> Result<(), String> {
        let c1: Class = Class::add_class("Swordsman");

        if c1 == Class::Swordsman {
            Ok(()) // pass
        } else {
            Err(String::from("Class does not match"))
        }
    }

    #[test]
    #[should_panic(expected = "Class does not exist")]
    fn class_creation_panic() {
        Class::add_class("Cleric");
    }

    #[test]
    fn world_character() -> Result<(), String> {
        let p1: Player = Player::new("James", "Thief");

        if p1.name == "James" && p1.class == Class::Thief {
            Ok(())
        } else {
            Err(String::from("Could not create character"))
        }
    }

    #[test]
    fn test_speech() {
        let p1 = Player::new("James", "Mage");
        assert_eq!(p1.speech(), "My name is James");
    }
}