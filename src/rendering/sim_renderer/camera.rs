use glam::{Vec2, Affine2};
use crate::rendering::Display;

pub struct Camera {
    pub pos: Vec2,
    pub scale: Vec2,
    pub size: Vec2
}

impl Camera {
    pub fn new(display: &Display) -> Self {
        let win_size = display.window.inner_size();
        Camera {
            pos: Vec2::ZERO,
            scale: Vec2::ONE,
            size: Vec2::new(win_size.width as f32, win_size.height as f32),
        }
    }

    pub fn translate_by(&mut self, delta: Vec2) {
        self.pos += delta;
    }
    
    pub fn translate_to(&mut self, delta: Vec2) {
        self.pos = delta;
    }

    pub fn scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }

    pub fn transform(&self, pos: Vec2) -> Vec2 {
        let tmp = Affine2::from_angle_translation(0.0, self.pos);
        tmp.transform_point2(pos)
    }

    /// Transform screen coordinate to world coordinate
    pub fn screen2world(&self, in_pos: Vec2) -> Vec2 {
        self.pos + in_pos - self.size / 2.0
    }
}