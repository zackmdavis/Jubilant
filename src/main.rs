#[macro_use] extern crate itertools;

mod card;
mod distribution;

fn main() {
    let deck = card::deal();
    println!("Hello Jubilant Hidden Information Fireworks Game world!");
    println!("{:?}", deck);
}
