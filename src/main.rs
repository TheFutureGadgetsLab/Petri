use simulation::Config;

mod rendering;
mod simulation;

use futures::executor::block_on;

fn main() {
    let config = Config::default();
    block_on(rendering::run::<rendering::SimRenderer, rendering::GUIRenderer>(config));
}
