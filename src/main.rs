extern crate rand;

use rand::thread_rng;
use rand::distributions::{ IndependentSample, Range };

fn main() {
    let range = Range::new(1, 6);

    loop {
        let (player_one_score, player_two_score) = roll_for_both_players(&range);
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

fn roll_for_both_players(range: &Range<i32>) -> (i32, i32) {
    let mut player_one_score = 0;
    let mut player_two_score = 0;

    for _ in 0..3 {
        let roll_for_player_one = range.ind_sample(&mut thread_rng());
        let roll_for_player_two = range.ind_sample(&mut thread_rng());

        println!("Player one rolled {}", roll_for_player_one);
        println!("Player two rolled {}", roll_for_player_two);

        player_one_score += roll_for_player_one;
        player_two_score += roll_for_player_two;
    }

    (player_one_score, player_two_score)
}
