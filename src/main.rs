use simulation::Config;

mod rendering;
mod simulation;

fn main() {
    let config = Config::default();
    pollster::block_on(rendering::run::<rendering::SimRenderer, rendering::GUIRenderer>(config));
}
