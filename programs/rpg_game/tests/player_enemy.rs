use rpg_game::*;

#[test]
fn enemy_health_reduction() -> Result<(), String> {
    let p1 = Player::new("James", "Mage");
    let mut e1 = Enemy::new("Dragon");

    p1.inflict_damage(&mut e1, 40);
    p1.speech();

    if let 60 = e1.health {
        Ok(())
    } else {
        Err(String::from("Damage was not registered"))
    }
}

#[test]
fn enemy_health_reduction2() -> Result<(), String> {
    let p1 = Player::new("James", "Mage");
    let mut e1 = Enemy::new("Dragon");
    
    p1.inflict_damage(&mut e1, 112);

    if let 0 = e1.health {
        Ok(())
    } else {
        Err(String::from("Damage was not registered"))
    }
}