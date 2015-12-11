extern crate rand;

use rand::Rng;

static mut STATISTICAL_STATE: bool = true;

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
        let roll_for_player_one = statistical_normalization(rng());
        let roll_for_player_two = statistical_normalization(rng());

        println!("Player one rolled {}", roll_for_player_one);
        println!("Player two rolled {}", roll_for_player_two);

        player_one_score += roll_for_player_one;
        player_two_score += roll_for_player_two;
    }

    (player_one_score, player_two_score)
}

fn rng() -> i32 {
    rand::thread_rng().gen_range(1, 7)
}

// Improves range distribution
fn statistical_normalization(n: i32) -> i32 {
    unsafe {
        STATISTICAL_STATE = !STATISTICAL_STATE;
        if STATISTICAL_STATE && n > 3 {
            rand::thread_rng().gen_range(1, 7)
        } else {
            n
        }
    }
}
