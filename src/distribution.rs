#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use card::{self, Card};


pub struct Distribution {
    backing: HashMap<Card, usize>
}

impl Distribution {
    pub fn new() -> Self {
        let deck = card::deal();
        let mut distribution = Distribution {
            backing: HashMap::with_capacity(deck.len())
        };
        for card in deck {
            let frequency = distribution.backing.entry(card).or_insert(0);
            *frequency += 1;
        }
        distribution
    }

    pub fn set(&mut self, card: Card) {
        let mut rebacking = HashMap::new();
        rebacking.insert(card, 1);
        self.backing = rebacking;
    }

    pub fn decrement(&mut self, card: Card) {
        if let Entry::Occupied(mut o) = self.backing.entry(card) {
            *o.get_mut() -= 1;
        }
    }

    pub fn probability(&self, card: Card) -> f64 {
        let total: usize = self.backing.values().sum();
        *self.backing.get(&card).unwrap_or(&0) as f64 / (total as f64)
    }

    pub fn entropy(&self) -> f64 {
        let values = self.backing.values().cloned().collect::<Vec<usize>>();
        let total = values.iter().sum::<usize>() as f64;
        values.iter().map(|d| {
            let f = (*d) as f64;
            -(f/total) * (f/total).log2()
        }).sum()
    }

}

#[cfg(test)]
#[macro_use]
macro_rules! assert_eq_within_epsilon {
    // crude edit of the canonical `assert_eq!`
    ($left:expr, $right:expr, $epsilon:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if (*left_val - *right_val).abs() > $epsilon {
                    panic!("assertion failed: left and right not within ε \
                           (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concerning_intial_entropy() {
        let distribution = Distribution::new();
        assert_eq_within_epsilon!(4.5683, distribution.entropy(), 0.0001);
    }
}
