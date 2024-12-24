#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Suit {
    Club,
    Spade,
    Heart,
    Diamond,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Rank {
    pub fn get_chips(&self) -> u128 {
        match self {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
}

pub struct Card {
    suit: Suit,
    rank: Rank,
    chips: u128,
}

impl Card {
    pub fn get_chips(&self) -> u128 {
        self.rank.get_chips() + self.chips
    }
}
