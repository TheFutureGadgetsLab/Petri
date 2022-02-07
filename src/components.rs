use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Default)]
pub struct CellMarker;

#[derive(Component, Default)]
pub struct ColorComp {
    pub val: [f32; 4],
}

impl ColorComp {
    pub fn new_rand(rng: &mut ThreadRng) -> Self {
        Self {
            val: [rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>(), 1.0],
        }
    }
}

#[derive(Bundle, Default)]
pub struct CellBundle {
    pub marker: CellMarker,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub color: ColorComp,
}
