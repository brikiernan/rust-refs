use rand::seq::SliceRandom;
use rand::{rng, Rng};

#[derive(Debug, Clone, Copy)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}

#[derive(Debug, Clone, Copy)]
enum Rank {
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
    Ace,
    Joker,
}

#[derive(Debug)]
struct Card {
    rank: Rank,
    suit: Option<Suit>,
}

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        let suits = [Suit::Clubs, Suit::Spades, Suit::Hearts, Suit::Diamonds];
        let ranks = [
            Rank::Two,
            Rank::Three,
            Rank::Four,
            Rank::Five,
            Rank::Six,
            Rank::Seven,
            Rank::Eight,
            Rank::Nine,
            Rank::Ten,
            Rank::Jack,
            Rank::Queen,
            Rank::King,
            Rank::Ace,
        ];

        let mut cards = Vec::with_capacity(52);

        for suit in suits.into_iter() {
            for rank in ranks.into_iter() {
                cards.push(Card {
                    rank,
                    suit: Some(suit),
                });
            }
        }

        println!("Deck created with {} cards.", cards.len());

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn insert_jokers(&mut self) {
        let mut rng = rng();

        for _ in 0..2 {
            let index = rng.random_range(0..self.cards.len());
            self.cards.insert(
                index,
                Card {
                    rank: Rank::Joker,
                    suit: None,
                },
            );
        }
    }

    fn delete_random_card(&mut self) {
        let mut rng = rng();

        if rng.random_bool(0.65) && !self.cards.is_empty() {
            let index = rng.random_range(0..self.cards.len());
            self.cards.remove(index);
        }
    }
}

fn main() {
    let mut deck = Deck::new();
    println!("----------------");
    println!("Initial deck: {:#?}", deck);
    println!("----------------");
    deck.shuffle();
    println!("Shuffled deck: {:#?}", deck);
    println!("----------------");
    deck.insert_jokers();
    println!("Deck after inserting jokers: {:#?}", deck);
    println!("----------------");
    for _ in 0..10 {
        deck.delete_random_card();
    }
    println!("Deck after deleting random cards: {:#?}", deck);
    println!("----------------");
    println!("Number of cards left in the deck: {}", deck.cards.len());
}
