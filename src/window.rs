use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, UpdateArgs};
use graphics::*;
use glutin_window::GlutinWindow;
use piston::window::WindowSettings;
use fps_counter::*;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const BLUE: [f32; 4]  = [0.0, 1.0, 1.0, 1.0];

pub struct Window {
    pub gl: GlGraphics,    // OpenGL drawing backend.
    pub rotation: f64,     // Rotation for the square.
    pub win: GlutinWindow, // Window to render to
    pub fps: FPSCounter,
}

impl Window {
    pub fn default() -> Window {
        let opengl = OpenGL::V4_5;
        Window {
            rotation: 0.0,
            win: WindowSettings::new("spinning-square", [200, 200])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap(),
            gl: GlGraphics::new(opengl),
            fps: FPSCounter::default()
        }
    }


    pub fn render(&mut self, args: &RenderArgs) {
        self.fps.tick();

        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            let rect = ellipse::circle(0., 0., 10.);
            let transform = c.transform.trans(x, y);
            ellipse(BLUE, rect, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}