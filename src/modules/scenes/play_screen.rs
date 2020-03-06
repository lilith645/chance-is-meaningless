use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::{LoadScreen};
use crate::cgmath::{Vector2, Vector4};

pub struct PlayScreen {
  data: SceneData,
}

impl PlayScreen {
  pub fn new() -> PlayScreen {
    PlayScreen {
      data: SceneData::new_default(),
    }
  }
}

impl Scene for PlayScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    Box::new(PlayScreen::new())
  }
  
  fn update(&mut self, delta_time: f32) {
    
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    draw_calls.push(
        DrawCall::draw_coloured(Vector2::new(width*0.5, height*0.5),
                                Vector2::new(width*5.0, height*5.0),
                                Vector4::new(1.0, 1.0, 1.0, 1.0),
                                0.0)
    );
    draw_calls.push(
      DrawCall::draw_textured(Vector2::new(width*0.45, height*0.6), 
                              Vector2::new(500.0, 500.0),
                              0.0,
                              String::from("Logo"))
    );
  }
}
