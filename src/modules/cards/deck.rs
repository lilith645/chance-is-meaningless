use rand::thread_rng;
use rand::prelude::*;

use crate::modules::cards::{Card, CardSuit};

use maat_graphics::DrawCall;

pub struct Deck {
  cards: Vec<Card>,
}

impl Deck {
  pub fn new_full() -> Deck {
    
    let mut cards = Deck::generate_suit(CardSuit::Hearts);
    cards.append(&mut Deck::generate_suit(CardSuit::Diamonds));
    cards.append(&mut Deck::generate_suit(CardSuit::Spades));
    cards.append(&mut Deck::generate_suit(CardSuit::Clubs));
    
    Deck {
      cards: Vec::new(),
    }
  }
  
  pub fn new_empty() -> Deck {
    Deck {
      cards: Vec::new(),
    }
  }
  
  pub fn add_card_to_bottom(&mut self, card: Card) {
    self.cards.push(card);
  }
  
  pub fn add_card_to_top(&mut self, card: Card) {
    self.cards.insert(0, card);
  }
  
  pub fn shuffle(&mut self, rng: &mut ThreadRng) {
    for i in 0..self.cards.len() {
      let j = (rng.gen::<f32>()*i as f32).floor() as usize;
      let t = self.cards[i].clone();
      self.cards[i] = self.cards[j].clone();
      self.cards[j] = t;
    }
  }
  
  pub fn take_top(&mut self) -> Option<Card> {
    let mut card = None;
    
    if self.cards.len() > 0 {
      card = Some(self.cards.remove(0));
    }
    
    card
  }
  
  pub fn reset_full(&mut self) {
    
    let mut cards = Deck::generate_suit(CardSuit::Hearts);
    cards.append(&mut Deck::generate_suit(CardSuit::Diamonds));
    cards.append(&mut Deck::generate_suit(CardSuit::Spades));
    cards.append(&mut Deck::generate_suit(CardSuit::Clubs));
    
    self.cards = cards;
  }
  
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
  
  pub fn draw(&self, x: f32, y: f32, draw_calls: &mut Vec<DrawCall>) {
    for card in &self.cards {
      card.draw(x, y, draw_calls);
    }
  }
}
