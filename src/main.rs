mod rendering;
mod simulation;

fn main() {
    pollster::block_on(rendering::run::<rendering::SimRenderer>());
}
