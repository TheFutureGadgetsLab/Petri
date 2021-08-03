pub mod render_framework;
mod renderer;

use renderer::Renderer;

fn main() {
    pollster::block_on(render_framework::run::<Renderer>());
}
