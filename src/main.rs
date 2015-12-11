extern crate rand;

use rand::Rng;

fn main() {
    loop {
        match (&roll(), &roll()) {
            (p1, p2) if p1 != p2 => {
                println!("{} wins!", if p1 > p2 { "Player 1" } else { "Player 2" });
                return;
            },
            _ => (),
        }
    }
}

fn roll() -> Vec<i32> {
    (0..3).map(|_| rng()).collect()
}

fn rng() -> i32 {
    rand::thread_rng().gen_range(1, 7)
}
