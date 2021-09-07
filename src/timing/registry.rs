#![macro_use]
use lazy_static::lazy_static;
use parking_lot::RwLock;

use super::timer::{Resolution, Timer};

lazy_static! {
    pub static ref TIMING_DATABASE: RwLock<GlobalTimers> = RwLock::new(GlobalTimers::default());
}

macro_rules! time_func {
    ($module:ident,$stage:ident,$start:ident) => {
        unsafe {
            TIMING_DATABASE
                .data_ptr()
                .as_mut()
                .unwrap()
                .$module
                .$stage
                .update(Instant::now() - $start);
        }
    };
}
pub(crate) use time_func;

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
            step: Timer::new(Resolution::Micro),
            pos_update: Timer::new(Resolution::Micro),
            grid_update: Timer::new(Resolution::Micro),
            col_detect: Timer::new(Resolution::Micro),
            col_resolve: Timer::new(Resolution::Micro),
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
            vertex_buffer_update: Timer::new(Resolution::Micro),
            render: Timer::new(Resolution::Micro),
        }
    }
}
