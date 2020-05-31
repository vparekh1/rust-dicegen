extern crate rand;

mod roll;
mod parser;

use rand::Rng;

fn main() {
    let rng = rand::thread_rng();
    let mut request = roll::RollRequest::new(10000, 1000, Some(500), rng);

    println!("{:?}", request.roll_dice());
}
