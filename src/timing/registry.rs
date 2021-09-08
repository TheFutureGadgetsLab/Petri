#![macro_use]
use lazy_static::lazy_static;
use parking_lot::RwLock;

use super::timer::Timer;

lazy_static! {
    pub static ref TIMING_DATABASE: RwLock<GlobalTimers> = RwLock::new(GlobalTimers::default());
}

#[derive(Default)]
pub struct GlobalTimers {
    pub physics: PhysicsTimers,
    pub gui_render: GUITimers,
    pub sim_render: SimRenderTimers,
}
#[derive(Default)]
pub struct GUITimers {
    pub draw: Timer,
}

#[derive(Default)]
pub struct PhysicsTimers {
    pub step: Timer,
    pub pos_update: Timer,
    pub col_detect: Timer,
}

#[derive(Default)]
pub struct SimRenderTimers {
    pub render: Timer,
    pub vertex_buffer_update: Timer,
}
