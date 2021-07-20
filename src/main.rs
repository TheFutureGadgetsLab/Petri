mod glass;

use glass::renderer::MainState;
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

    let hidpi_factor = event_loop
        .primary_monitor().unwrap()
        .scale_factor() as f32;
    println!("main hidpi_factor = {}", hidpi_factor);

    let state = MainState::new(&mut ctx, hidpi_factor)?;

    event::run(ctx, event_loop, state)
}
