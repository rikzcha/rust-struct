pub mod player;

use crate::player::Player;
use crate::player::FullName;


fn main() {

    let player_1 = Player::new("Lionel".to_string(), "Messi".to_string());
    let player_empty = Player::default();

    println!("Player 1: {}", player_1.full_name());

    let mut player_2 = Player::update_first_name("Christiano".to_string(), player_empty);
    player_2 = Player::update_last_name("Ronaldo".to_string(), player_2);

    println!("Player 2: {}", player_2.full_name());

    player_2 = Player::update(player_1);

    println!("Player 2: {}", player_2.full_name());

}


