use piston::EventLoop;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
mod window;


fn main() {
    // Create a new game and run it.
    let mut app = window::Window::default();

    let mut events = Events::new(EventSettings::new());
    events.set_bench_mode(true);

    while let Some(e) = events.next(&mut app.win) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
