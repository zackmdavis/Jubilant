#[macro_use] extern crate itertools;
extern crate rand;

mod card;
mod distribution;
mod game;
mod player;

fn main() {
    let deck = card::deal();
    println!("Hello Jubilant Hidden Information Fireworks Game world!");
    println!("{:?}", deck);
}
