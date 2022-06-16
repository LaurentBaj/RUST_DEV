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
pub enum EType {
    Lurker,
    Orc,
    Dragon,
    Goblin   
}

impl EType {
    fn add_etype(input: &str) -> EType {
        match input {
            "Lurker" => EType::Lurker,
            "Dragon" => EType::Dragon,
            "Goblin" => EType::Goblin,
            "Orc" => EType::Orc,
            _=> panic!("Enemy type does not exist")
        }
    }
    fn str_enemy(&self) -> &str {
        match &self {
            EType::Lurker => "Lurker",
            EType::Dragon => "Dragon",
            EType::Goblin => "Goblin",
            EType::Orc => "Orc"
        }
    }
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

impl Player {
    fn inflict_damage(&self, enemy: &mut Enemy, damage: u8) {

        // If player damage is greater than enemy health
        if damage > enemy.health {
            enemy.health = 0;
        } else {
            enemy.health = enemy.health - damage;
        }
    }
}

impl Entity for Player {
    fn speech(&self) -> String {
        let mut preamble: String = String::from("My name is ");
        preamble.push_str(&self.name);
        preamble
    }
}

#[derive(Debug, PartialEq)]
pub struct Enemy {
    etype: EType,
    health: u8
}

impl Enemy {
    fn new(input: &str) -> Enemy {
        Enemy {
            etype: EType::add_etype(input),
            health: 100
        }
    }
}

impl Entity for Enemy {
    fn speech(&self) -> String { 
        let mut preamble: String = String::from("Beware! For I am ");
        preamble.push_str(EType::str_enemy(&self.etype));
        preamble
    }
}



// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    // Test Class Enum
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

    // Test Plater struct
    #[test]
    fn player_creation() -> Result<(), String> {
        let p1: Player = Player::new("James", "Thief");

        if p1.name == "James" && p1.class == Class::Thief {
            Ok(())
        } else {
            Err(String::from("Could not create character"))
        }
    }

    #[test]
    fn test_speech() {
        let p1 = Player::new("James", "Thief");
        assert_eq!(p1.speech(), String::from("My name is James"));
    }

    // Test Enemy struct + EType enum
    #[test]
    #[should_panic(expected = "Enemy type does not exist")]
    fn enemy_type() {
        let e1 = EType::add_etype("Lurker");
        let e2 = EType::add_etype("Rouge Ninja");

        assert_eq!(e1, EType::Lurker);
        assert_eq!(e2, EType::Lurker);
    }

    
    #[test]
    fn enemy_creation() -> Result<(), String> {
        let e1 = Enemy::new("Dragon");
        
        if e1.etype == EType::Dragon && e1.health == 100 {
            Ok(())
        } else {
            Err(String::from("Error creating enemy"))
        }
    }

    #[test]
    fn enemy_health_reduciton() -> Result<(), String> {
        let p1 = Player::new("James", "Mage");
        let mut e1 = Enemy::new("Dragon");
        
        p1.inflict_damage(&mut e1, 40);

        if let 60 = e1.health {
            Ok(())
        } else {
            Err(String::from("Damage was not registered"))
        }
    }
}