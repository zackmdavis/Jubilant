#![allow(dead_code)]

use std::rc::Rc;

use distribution::Distribution;
use game::GameState;
use card::Card;


/// A representation about what a player believes about the players' hands.
struct GameStateModel {
    hands: Vec<Vec<Distribution>>,
}

impl GameStateModel {
    fn new(n: usize) -> Self {
        let mut hands = Vec::new();
        for _ in 0..n {
            let mut hand = Vec::new();
            for _ in 0..6 {
                hand.push(Distribution::new());
            }
            hands.push(hand)
        }
        GameStateModel { hands }
    }

    fn rule_out(&mut self, player_index: usize, card: Card) {
        for hand in self.hands[player_index].iter_mut() {
            hand.rule_out(card);
        }
    }
}

pub struct PlayerInstance {
    self_index: usize,
    models: Vec<GameStateModel>,
    game: Rc<GameState>
}

impl PlayerInstance {
    fn new(game: Rc<GameState>, self_index: usize) -> Self {
        let n = game.n_players();
        let mut models = Vec::new();
        for _ in 0..n {
            models.push(GameStateModel::new(n));
        }
        PlayerInstance { game, self_index, models }
    }
}
