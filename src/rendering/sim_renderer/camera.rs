use glam::{Vec2, vec2};
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
            size: vec2(win_size.width as f32, win_size.height as f32),
        }
    }

    /// Transform world position and scale to render position and scale
    pub fn transform(&self, in_pos: Vec2, in_scale: Vec2) -> (Vec2, Vec2) {
        let p = (self.pos + in_pos) / self.size;
        (vec2(-p.x, p.y), self.scale * in_scale)
    }

    /// Transform screen coordinate to world coordinate
    pub fn screen2world(&self, in_pos: Vec2) -> Vec2 {
        self.pos + in_pos - self.size / 2.0
    }
}