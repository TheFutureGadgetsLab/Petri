mod glass;

use glass::renderer::Renderer;
use ggez::{conf, event};


pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("super_simple with imgui", "ggez")
        .window_setup(
            conf::WindowSetup::default()
                .title("super_simple with imgui")
                .vsync(false),
        )
        .window_mode(
            conf::WindowMode::default().resizable(true), /*.dimensions(750.0, 500.0)*/
        );
    let (mut ctx, event_loop) = cb.build()?;
    let state = Renderer::new(&mut ctx, &event_loop)?;

    event::run(ctx, event_loop, state)
}
