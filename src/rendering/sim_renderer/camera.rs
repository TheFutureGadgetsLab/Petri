use glam::{vec2, Vec2};

pub struct Camera {
    pub window_size: Vec2,
    /// Translation which is used to transform objects in space
    pub translation: Vec2,
    pub zoom: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(window_size: Vec2) -> Self {
        Camera {
            window_size,
            translation: Vec2::ZERO,
            zoom: 1.0,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.window_size = Vec2::new(width as _, height as _);
    }

    pub fn translate_by(&mut self, delta: Vec2) {
        self.translation += (delta * 2.0) / self.zoom;
    }

    /// Position of camera (center) in world space
    pub fn pos(&self) -> Vec2 {
        -self.translation / 2.0
    }

    pub fn screen2world(&self, p: Vec2) -> Vec2 {
        self.pos() + ((p - (self.window_size / 2.0)) * vec2(1.0, -1.0)) / self.zoom
    }
}
