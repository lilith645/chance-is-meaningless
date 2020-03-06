
use crate::modules::cards::{Card, CardSuit};

pub struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Deck {
    
    let mut cards = Deck::generate_suit(CardSuit::Hearts);
    cards.append(&mut Deck::generate_suit(CardSuit::Diamonds));
    cards.append(&mut Deck::generate_suit(CardSuit::Spades));
    cards.append(&mut Deck::generate_suit(CardSuit::Clubs));
    
    Deck {
      cards: Vec::new(),
    }
  }
  
  pub fn new_empty() {
    
  }
  
  pub fn shuffle(&mut self) {
    
  }
  /*
  pub fn take_top(&mut self) -> Card {
    
  }*/
  
  fn generate_suit(suit: CardSuit) -> Vec<Card> {
    let mut cards = Vec::new();
    
    cards.push(Card::new().two(&suit));
    cards.push(Card::new().three(&suit));
    cards.push(Card::new().four(&suit));
    cards.push(Card::new().five(&suit));
    cards.push(Card::new().six(&suit));
    cards.push(Card::new().seven(&suit));
    cards.push(Card::new().eight(&suit));
    cards.push(Card::new().nine(&suit));
    cards.push(Card::new().ten(&suit));
    cards.push(Card::new().jack(&suit));
    cards.push(Card::new().queen(&suit));
    cards.push(Card::new().king(&suit));
    cards.push(Card::new().ace(&suit));
    
    cards
  }
}
