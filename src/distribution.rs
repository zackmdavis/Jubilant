#![allow(dead_code)]

use std::collections::HashMap;

use card::{self, Card};


pub struct Distribution {
    // XXX: actually, probabilities are overkill: it's crisper to keep track of
    // which cards are possible, and derive a probability from that if
    // necessary
    backing: HashMap<Card, f64>
}

impl Distribution {
    pub fn new() -> Self {
        let deck = card::deal();
        let density = 1./(deck.len() as f64);
        let mut distribution = Distribution {
            backing: HashMap::with_capacity(deck.len())
        };
        for card in deck {
            let p = distribution.backing.entry(card).or_insert(0.);
            *p += density;
        }
        distribution
    }

    pub fn entropy(&self) -> f64 {
        self.backing.values().map(|p| -p * p.log2()).sum()
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
    fn concerning_conservation_of_probability_after_card_exclusion() {
        let distribution = Distribution::new();
        assert_eq_within_epsilon!(1., distribution.backing.values().sum::<f64>(),
                                  0.000001);
    }

}
