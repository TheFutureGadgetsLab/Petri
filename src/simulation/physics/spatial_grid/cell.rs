use legion::Entity;
use parking_lot::RwLock;

use crate::vec2::Vec2;

#[derive(Default)]
pub struct Cell {
    /// (new cell, entity, new pos)
    to_move: RwLock<Vec<(usize, Vec2, Entity)>>,
    ents: RwLock<Vec<(Vec2, Entity)>>,
}

impl Cell {
    pub fn insert(&self, pos: Vec2, entity: Entity) {
        self.ents.write().push((pos, entity));
    }

    pub fn clear(&self) {
        self.to_move.write().clear();
        self.ents.write().clear();
    }

    pub fn unlock_unsafe(&self) -> &Vec<(Vec2, Entity)> {
        unsafe { self.ents.data_ptr().as_ref().unwrap() }
    }

    pub fn unlock_unsafe_mut(&self) -> &mut Vec<(Vec2, Entity)> {
        unsafe { self.ents.data_ptr().as_mut().unwrap() }
    }
}
