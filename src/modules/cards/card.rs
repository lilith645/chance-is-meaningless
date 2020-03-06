
use maat_graphics::DrawCall;
use maat_graphics::cgmath::{Vector2, Vector4};

#[derive(PartialEq, Clone)]
pub enum CardSuit {
  Hearts,
  Diamonds,
  Spades,
  Clubs
}

#[derive(PartialEq, Clone)]
pub enum CardType {
  Number(u32),
  Ace,
  Jack,
  Queen,
  King
}

#[derive(Clone)]
pub struct Card {
  c_suit: CardSuit,
  c_type: CardType,
  facedown: bool,
  texture: String,
}

impl Card {
  pub fn new() -> Card {
    Card {
      c_suit: CardSuit::Hearts,
      c_type: CardType::Number(1),
      facedown: true,
      texture: "".to_string(),
    }
  }
  
  pub fn suit(&self) -> CardSuit {
    self.c_suit.clone()
  }
  
  pub fn c_type(&self) -> CardType {
    self.c_type.clone()
  }
  
  pub fn is_same_suit(&self, other: &Card) -> bool {
    self.suit() == other.suit()
  }
  
  pub fn is_higher(&self, other: &Card) -> bool {
    let mut self_is_higher = false;
    
    match &self.c_type {
      self_type => {
        match other.c_type() {
          face_type => {
            self_is_higher = self.compare_face_cards(other);
          },
          CardType::Number(y) => { } //other is number and is smaller
        }
      },
      CardType::Number(x) => {
        match other.c_type() {
          face_type => {
            self_is_higher = false;
          },
          CardType::Number(y) => {
            if x <= &y {
              self_is_higher = false;
            }
          }
        }
      },
    }
    
    self_is_higher
  }
  
  pub fn flip(&mut self) {
    if self.facedown {
      self.facedown = false;
    } else {
      self.facedown = true;
    }
  }
  
  fn compare_face_cards(&self, other: &Card) -> bool {
    let mut self_is_higher = true;
    match self.c_type() {
      CardType::Ace => {
        match other.c_type() {
          CardType::Ace => {
            self_is_higher = false;
          },
          _ => {},
        }
      },
      CardType::King => {
        match other.c_type() {
          CardType::Ace => {
            self_is_higher = false;
          },
          CardType::King => {
            self_is_higher = false;
          },
          _ => {},
        }
      },
      CardType::Queen => {
        match other.c_type() {
          CardType::Ace => {
            self_is_higher = false;
          },
          CardType::King => {
            self_is_higher = false;
          },
          CardType::Queen => {
            self_is_higher = false;
          },
          _ => {},
        }
      },
      CardType::Jack => {
        self_is_higher = false;
      },
      _ => {},
    }
    
    self_is_higher
  }
  
  fn add_suit(mut name: String, suit: &CardSuit) -> String {
    match suit {
      CardSuit::Hearts => {
        name += "_of_hearts";
      },
      CardSuit::Diamonds => {
        name += "_of_diamonds";
      },
      CardSuit::Spades => {
        name += "_of_spades";
      },
      CardSuit::Clubs => {
        name += "_of_clubs";
      }
    }
    
    name
  }
  
  pub fn ace(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("Ace".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Ace,
      facedown: true,
      texture,
    }
  }
  
  pub fn two(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("two".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(2),
      facedown: true,
      texture,
    }
  }
  
  pub fn three(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("three".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(3),
      facedown: true,
      texture,
    }
  }
  
  pub fn four(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("four".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(4),
      facedown: true,
      texture,
    }
  }
  
  pub fn five(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("five".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(5),
      facedown: true,
      texture,
    }
  }
  
  pub fn six(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("six".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(6),
      facedown: true,
      texture,
    }
  }
  
  pub fn seven(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("seven".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(7),
      facedown: true,
      texture,
    }
  }
  
  pub fn eight(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("eight".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(8),
      facedown: true,
      texture,
    }
  }
  
  pub fn nine(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("nine".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(9),
      facedown: true,
      texture,
    }
  }
  
  pub fn ten(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("ten".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Number(10),
      facedown: true,
      texture,
    }
  }
  
  pub fn jack(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("jack".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Jack,
      facedown: true,
      texture,
    }
  }
  
  pub fn queen(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("queen".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::Queen,
      facedown: true,
      texture,
    }
  }
  
  pub fn king(mut self, suit: &CardSuit) -> Card {
    let texture = Card::add_suit("king".to_string(), suit);
    
    Card {
      c_suit: suit.clone(),
      c_type: CardType::King,
      facedown: true,
      texture,
    }
  }
  
  pub fn draw(&self, x: f32, y: f32, draw_calls: &mut Vec<DrawCall>) {
    
    let mut width = 64.0;
    let mut height = width*1.605;
    
    if !self.facedown {
      draw_calls.push(DrawCall::draw_textured(Vector2::new(x, y), Vector2::new(width,height), 0.0, "cardback".to_string()));
    } else {
      draw_calls.push(DrawCall::draw_textured(Vector2::new(x, y), Vector2::new(width,height), 0.0, self.texture.to_string()));
    }
  }
}
