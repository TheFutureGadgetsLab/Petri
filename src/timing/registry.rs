use lazy_static::lazy_static;
use parking_lot::RwLock;

use super::timer::{Resolution, Timer};

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

pub struct PhysicsTimers {
    pub step: Timer,
    pub pos_update: Timer,
    pub grid_update: Timer,
    pub col_detect: Timer,
    pub col_resolve: Timer,
}

impl Default for PhysicsTimers {
    fn default() -> Self {
        Self {
            step: Timer::new(Resolution::MICRO),
            pos_update: Timer::new(Resolution::MICRO),
            grid_update: Timer::new(Resolution::MICRO),
            col_detect: Timer::new(Resolution::MICRO),
            col_resolve: Timer::new(Resolution::MICRO),
        }
    }
}

pub struct SimRenderTimers {
    pub render: Timer,
    pub vertex_buffer_update: Timer,
}

impl Default for SimRenderTimers {
    fn default() -> Self {
        Self {
            vertex_buffer_update: Timer::new(Resolution::MICRO),
            render: Timer::new(Resolution::MICRO),
        }
    }
}
