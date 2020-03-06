pub use crate::modules::cards::Deck;

use maat_graphics::cgmath::{Vector2, Vector4};
use maat_graphics::DrawCall;

const BOARD_HEIGHT: f32 = 0.20;
const BOARD_WIDTH: f32 = 0.60;

pub enum BoardPosition {
  Top,
  Bottom,
  Left,
  Right,
}

pub struct Board {
  deck: Deck,
  position: BoardPosition,
}

impl Board {
  pub fn new_bottom() -> Board {
    Board {
      deck: Deck::new_empty(),
      position: BoardPosition::Bottom,
    }
  }
  
  pub fn new_top() -> Board {
    Board {
      deck: Deck::new_empty(),
      position: BoardPosition::Top,
    }
  }
  
  pub fn mut_deck(&mut self) -> &mut Deck {
    &mut self.deck
  }
  
  pub fn draw(&self, width: f32, height: f32, draw_calls: &mut Vec<DrawCall>) {
    
    match &self.position {
      Bottom => {
        let center_board_x = width*0.5;
        let center_board_y = height*BOARD_HEIGHT*0.5;
        let b_width = BOARD_WIDTH*width;
        let b_height = BOARD_HEIGHT*height;
        
        let deck_right_x = center_board_x + width*0.2;
        let deck_right_y = center_board_y;
        
        // Board background
        draw_calls.push(
            DrawCall::draw_coloured(Vector2::new(center_board_x, center_board_y),
                                    Vector2::new(b_width, b_height),
                                    Vector4::new(0.0, 0.0, 0.33, 1.0),
                                    0.0)
        );
        
        self.deck.draw(deck_right_x, deck_right_y, draw_calls);
      },
      _ => {
        
      }
    }
  }
}
