#![allow(dead_code)]

use rand::{self, Rng};

use card::{self, Card};
use player::PlayerInstance;

#[derive(Copy, Clone, Default)]
pub struct VictoryArsenal {
    pub red: u8,
    pub yellow: u8,
    pub green: u8,
    pub blue: u8,
    pub white: u8
}

impl VictoryArsenal {
    fn color(&self, color: card::Color) -> u8 {
        match color {
            card::Color::Red => self.red,
            card::Color::Yellow => self.yellow,
            card::Color::Green => self.green,
            card::Color::Blue => self.blue,
            card::Color::White => self.white,
        }
    }

    fn bump_color(&mut self, color: card::Color) {
        match color {
            card::Color::Red => { self.red += 1; },
            card::Color::Yellow => { self.yellow += 1; },
            card::Color::Green => { self.green += 1; },
            card::Color::Blue => { self.blue += 1; },
            card::Color::White => { self.white += 1; },
        }
    }
}

pub trait Player {
    fn go(&mut self) -> Action;
    fn observe(&mut self, action: Action);
}

#[derive(Copy, Clone)]
pub enum HintContent {
    Color(card::Color),
    Value(card::Value)
}

#[derive(Copy, Clone)]
pub enum Action {
    Hint{
        player: usize, // index into players
        cards: usize, // index into hands[player]
        hint_content: HintContent
    },
    Discard {
        player: usize,
        card: usize
    },
    Play {
        player: usize,
        card: usize
    }
}

pub struct GameState {
    players: Vec<Box<Player>>,
    hands: Vec<Vec<Card>>,
    deck: Vec<Card>,
    pub turn_index: usize, // index into players
    pub victory_arsenal: VictoryArsenal,
    pub hints: u8,
    pub fuse: u8,
}

#[derive(Copy, Clone)]
pub struct TableState {
    pub turn_index: usize,
    pub victory_arsenal: VictoryArsenal,
    pub hints: u8,
    pub fuse: u8,
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

        let players = (0..n).map(|i| {
            Box::new(
                PlayerInstance::new(
                    hands.iter().cloned().enumerate().map(|(j, h)| {
                        if i == j {
                            None
                        } else {
                            Some(h)
                        }
                    }).collect()
                )
            ) as Box<Player>
        }).collect();

        Self {
            players,
            hands,
            deck,
            turn_index: 0,
            victory_arsenal: VictoryArsenal::default(),
            hints: 8,
            fuse: 3
        }
    }

    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn table_state(&self) -> TableState {
        TableState {
            turn_index: self.turn_index,
            victory_arsenal: self.victory_arsenal,
            hints: self.hints,
            fuse: self.fuse
        }
    }

    fn apply(&mut self, player_index: usize, action: Action) {
        match action {
            Action::Hint { .. } => {
                self.hints -= 1;
            },
            Action::Discard { player, card } => {
                self.hands[player].swap_remove(card);
                self.hints += 1;
                self.draw(player_index);
            },
            Action::Play { player, card } => {
                let card = self.hands[player].swap_remove(card);
                let current = self.victory_arsenal.color(card.color);
                if card.value == current + 1 {
                    self.victory_arsenal.bump_color(card.color)
                } else {
                    self.fuse -= 1;
                }
                self.draw(player_index);
            }
        }
    }

    fn draw(&mut self, player_index: usize) {
        let new_card = self.deck.pop()
            // XXX TODO deck will be empty at end of game; rework as `Result`
            .expect("deck shouldn't be empty");
        self.hands[player_index].push(new_card);
    }

    pub fn turn(&mut self) {
        let turn_index = self.turn_index;
        let action = self.players[self.turn_index].go();
        for player_index in 0..self.players.len() {
            if player_index != turn_index {
                self.players[player_index].observe(action);
            }
        }
        self.apply(turn_index, action);
        self.turn_index = (self.turn_index + 1) % self.players.len();
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
