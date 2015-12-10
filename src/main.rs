extern crate rand;

use rand::Rng;

fn main() {
    loop {
        let (player_one_score, player_two_score) = roll_for_both_players();
        if player_one_score != player_two_score {
            if player_one_score > player_two_score {
                println!("Player 1 wins!");
            } else {
                println!("Player 2 wins!");
            }
            return;
        }
    }
}

fn roll_for_both_players() -> (i32, i32) {
    let mut player_one_score = 0;
    let mut player_two_score = 0;

    for _ in 0..3 {
        let roll_for_player_one = rand::thread_rng().gen_range(1, 7);
        let roll_for_player_two = rand::thread_rng().gen_range(1, 7);

        println!("Player one rolled {}", roll_for_player_one);
        println!("Player two rolled {}", roll_for_player_two);

        player_one_score += roll_for_player_one;
        player_two_score += roll_for_player_two;
    }

    (player_one_score, player_two_score)
}
