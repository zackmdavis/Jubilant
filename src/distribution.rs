#![allow(dead_code)]

use std::collections::HashMap;

use card::{self, Card};


pub struct Distribution {
    backing: HashMap<Card, f64>
}

impl Distribution {
    pub fn uniform() -> Self {
        let deck = card::deal();
        let density = 1./(deck.len() as f64);
        let mut distribution = Distribution {
            backing: HashMap::with_capacity(deck.len())
        };
        for card in deck {
            distribution.backing.insert(card, density);
        }
        distribution
    }

    pub fn entropy(&self) -> f64 {
        self.backing.values().map(|p| -p * p.log2()).sum()
    }
}
