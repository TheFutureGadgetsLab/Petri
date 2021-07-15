use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
mod app;
use fps_counter::*;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    // Create a new game and run it.
    let mut app = app::App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut a = FPSCounter::default();
    let mut i = 0;

    let mut events = Events::new(EventSettings::new());
    events.set_bench_mode(true);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
            let fps = a.tick();
            i += 1;
            if (i % 1000) == 0 {
                println!("{:?}", fps);
            }
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
