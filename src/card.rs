use std::iter;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
    White,
}

impl Color {
    pub fn colors() -> Vec<Self> {
        vec![Color::Red, Color::Yellow, Color::Green, Color::Blue, Color::White]
    }
}

pub type Value = usize;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub color: Color,
    pub value: Value,
}

impl Card {
    fn new(color: Color, value: Value) -> Self {
        Self { color, value }
    }
}

pub fn deal() -> Vec<Card> {
    let quantities = vec![(1, 3), (2, 2), (3, 2), (4, 2), (5, 1)];
    iproduct!(Color::colors(), quantities)
        .flat_map(|(color, (value, quantity))| {
            iter::repeat(Card::new(color, value)).take(quantity)
        }).collect()
}
