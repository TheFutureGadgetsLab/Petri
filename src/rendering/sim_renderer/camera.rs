use glam::{Vec2};
use crate::rendering::Display;

static W2S_FAC: f32 = 1.0 / 10000.0;

pub struct Camera {
    pub scale: Vec2,
    pub window_size: Vec2,
    pub translation: Vec2,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(display: &Display) -> Self {
        let size = Vec2::new(
         display.window.inner_size().width as _,
         display.window.inner_size().height as _
        );

        let scale =  Vec2::new(
            size.y * W2S_FAC,
            size.x * W2S_FAC,
        ) / size.min_element();

        Camera {
            scale,
            window_size: size,
            translation: Vec2::ZERO,
        }
    }

    pub fn translate_by(&mut self, delta: Vec2) {
        self.translation += delta * 50.0 * W2S_FAC;
    }
    
    pub fn transform(&self, pos: Vec2) -> Vec2 {
        self.translation + self.scale * pos
    }
    
    pub fn rescale_window(&mut self, _scale: Vec2) {
        //self.scale = scale;
    }

    pub fn screen_to_world(&self, pos: Vec2) -> Vec2 {
        pos / self.scale - self.translation
    }
}