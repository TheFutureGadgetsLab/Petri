use glam::Vec2;

pub struct Camera {
    pub window_size: Vec2,
    /// Center position of camera
    pub translation: Vec2,
    pub zoom: f32,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(window_size: Vec2) -> Self {
        Camera {
            window_size: window_size,
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

    pub fn screen2world(&self, p: Vec2) -> Vec2 {
        p - (self.translation + self.window_size / 2.0)
    }
}
