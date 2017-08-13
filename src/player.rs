#![allow(dead_code)]

use distribution::Distribution;
use card::Card;
use game::{Action, Player};


/// A representation about what a player believes about the players' hands.
struct GameStateModel {
    hands: Vec<Vec<Distribution>>,
    // XXX TODO: need to model the deck, too
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

    // maybe rename `account_for` or similar??
    fn decrement(&mut self, player_index: usize, card: Card) {
        for hand in self.hands[player_index].iter_mut() {
            hand.decrement(card);
        }
    }

}

pub struct PlayerInstance {
    self_index: usize,
    models: Vec<GameStateModel>,
}

impl PlayerInstance {
    pub fn new(initial_view: Vec<Option<Vec<Card>>>) -> Self {
        let n = initial_view.len();
        let mut models = Vec::new();
        for _ in 0..n {
            models.push(GameStateModel::new(n));
        }

        let self_index = initial_view.iter().position(|v| v.is_none())
            .expect("one hand view should be empty to indicate player");
        for (hand_index, hand_view) in initial_view.into_iter().enumerate() {
            if let Some(view) = hand_view {
                for (card_index, card) in view.iter().enumerate() {
                    for player_index in 0..n {
                        if player_index != hand_index {
                            // Everyone _else_ sees the cards
                            models[player_index].hands[hand_index][card_index]
                                .set(*card);
                            // ... and can make probabilistic inferences about
                            // their own cards
                            models[player_index].decrement(player_index, *card);
                        }
                    }
                }
            }
        }

        PlayerInstance { self_index, models }
    }
}

impl Player for PlayerInstance {
    fn go(&mut self) -> Action {
        // XXX TODO but the following line at least typechecks
        Action::Discard { player: self.self_index, card: 0 }
    }

    fn observe(&mut self, _action: Action) {
        // XXX TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concerning_initial_knowledge() {
        let prior_distribution = Distribution::new();
        // show entropically that we know more than nothing about our own
        // cards, but are certain about the cards we can see
    }

}
