extern crate rand;

use rand::Rng;

fn main() {
    loop {
        let (p1, p2) = (roll(), roll());
        if p1 != p2 {
            println!("{} wins!", if p1 > p2 { "Player 1" } else { "Player 2" });
            return;
        }
    }
}

fn roll() -> Vec<i32> {
    (0..3).map(|_| rng()).collect()
}

fn rng() -> i32 {
    rand::thread_rng().gen_range(1, 7)
}
