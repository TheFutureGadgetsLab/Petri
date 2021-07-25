use core::f32;
use ggez;
use ggez::graphics::DrawParam;
use glam::Vec2;

pub struct Camera {
    pub transform: DrawParam,
    pos: Vec2,
    scale: Vec2,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            transform: DrawParam::default(),
            pos: Vec2::ZERO,
            scale: Vec2::ONE
        }
    }

    pub fn move_by(&mut self, by: Vec2) {
        self.pos += by;
        self.transform = self.transform.dest(self.pos);
    }

    pub fn zoom_by(&mut self, by: f32) {
        self.scale *= by * (1.0 - by.signum() * 0.1);
        self.transform = self.transform.scale(self.scale);
    }
}