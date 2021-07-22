mod glass;

use glass::renderer::Renderer;
use ggez::{conf, event};


pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("Petri", "Haydn Jones, Benjamin Mastripolito")
        .window_setup(
            conf::WindowSetup::default()
                .title("Petri")
                .vsync(false),
        )
        .window_mode(
            conf::WindowMode::default().resizable(true), /*.dimensions(750.0, 500.0)*/
        );
    let (mut ctx, event_loop) = cb.build()?;
    let renderer = Renderer::new(&mut ctx, &event_loop)?;

    event::run(ctx, event_loop, renderer)
}
