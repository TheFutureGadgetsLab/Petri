use ultraviolet::Vec2;

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
            translation: Vec2::zero(),
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
        self.pos() + ((p - (self.window_size / 2.0)) * Vec2::new(1.0, -1.0)) / self.zoom
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Default)]
pub struct CameraUniform {
    u_translation: [f32; 2],
    u_window_size: [f32; 2],
    u_zoom: [f32; 2],
}

impl From<&Camera> for CameraUniform {
    fn from(cam: &Camera) -> Self {
        CameraUniform {
            u_translation: cam.translation.into(),
            u_window_size: cam.window_size.into(),
            u_zoom: [cam.zoom; 2],
        }
    }
}
