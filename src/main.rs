mod rendering;

fn main() {
    pollster::block_on(rendering::run::<rendering::SimRenderer>());
}
