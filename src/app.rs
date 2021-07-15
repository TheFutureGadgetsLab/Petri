use opengl_graphics::{GlGraphics};
use piston::input::{RenderArgs, UpdateArgs};
use graphics::*;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub rotation: f64,  // Rotation for the square.
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let green = [0.0, 1.0, 1.0, 1.0];
            let rect = ellipse::circle(x, y, 10.);
            let transform = c
                .transform
                .rot_rad(rotation);
            ellipse(green, rect, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        self.rotation = 0.0;
    }
}