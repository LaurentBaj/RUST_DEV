use rpg_game::Enemy;
use rpg_game::world_character;

fn main() {
    let e1 = Enemy::new("Orc", "Swordsman");

    println!("{:?}", world_character(&e1));
}
