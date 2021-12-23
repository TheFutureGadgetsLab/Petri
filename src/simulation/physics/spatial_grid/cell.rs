use legion::Entity;
use spin::RwLock;
use ultraviolet::Vec2;

#[derive(Default)]
pub struct Cell {
    ents: RwLock<Vec<(Vec2, Entity)>>,
}

impl Cell {
    pub fn insert(&self, pos: Vec2, entity: Entity) {
        self.ents.write().push((pos, entity));
    }

    pub fn clear(&self) {
        self.ents.write().clear();
    }

    pub fn unlock_unsafe(&self) -> &Vec<(Vec2, Entity)> {
        unsafe { self.ents.as_mut_ptr().as_ref().unwrap() }
    }
}
