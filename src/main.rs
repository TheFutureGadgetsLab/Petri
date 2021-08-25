mod rendering;
mod simulation;
mod driver;

use futures::executor::block_on;
use simulation::Config;

fn main() {
    let config = Config::default();
    block_on(driver::run(config));
}
