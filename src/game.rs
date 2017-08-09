#![allow(dead_code)]

use rand::{self, Rng};

use card::{self, Card};

#[derive(Copy, Clone, Default)]
pub struct VictoryArsenal {
    pub red: usize,
    pub yellow: usize,
    pub green: usize,
    pub blue: usize,
    pub white: usize
}

pub trait Player {
    // TODO
}

struct GameState {
    players: Vec<Box<Player>>,
    hands: Vec<Vec<Card>>,
    deck: Vec<Card>,
    pub discard_pile: Vec<Card>,
    pub victory_arsenal: VictoryArsenal,
}


impl GameState {
    pub fn new(n: usize) -> Self {
        let mut deck = card::deal();
        rand::thread_rng().shuffle(deck.as_mut_slice());
        let mut hands = Vec::new();
        for _ in 0..n {
            let mut hand = Vec::new();
            for _ in 0..6 {
                hand.push(deck.pop()
                          .expect("deck should have enough for initial deal"));
            }
            hands.push(hand);
        }
        Self {
            players: Vec::new(), // TODO
            hands,
            deck,
            discard_pile: Vec::new(),
            victory_arsenal: VictoryArsenal::default()
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concerning_a_new_game() {
        GameState::new(4);
    }

}
