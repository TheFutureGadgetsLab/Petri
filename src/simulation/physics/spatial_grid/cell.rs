use glam::Vec2;

use super::grid::{GridHandle, GridObjects, ObjectState};

pub type CellObject = (GridHandle, Vec2);

/// A single cell of the grid, can be empty
#[derive(Default, Clone)]
pub struct GridCell {
    pub objs: Vec<CellObject>,
    pub dirty: bool,
}

impl GridCell {
    pub fn maintain(&mut self, objects: &mut GridObjects, to_relocate: &mut Vec<CellObject>) {
        if !self.dirty {
            return;
        }
        self.dirty = false;

        let mut i = 0;
        while i < self.objs.len() {
            let (obj_id, obj_pos) = unsafe { self.objs.get_unchecked_mut(i) };

            let store_obj = &mut objects[*obj_id];

            match store_obj.state {
                ObjectState::NewPos(pos) => {
                    store_obj.state = ObjectState::Unchanged;
                    store_obj.pos = pos;
                    *obj_pos = pos;
                    i += 1
                }
                ObjectState::Relocate(pos, target_id) => {
                    store_obj.state = ObjectState::Unchanged;
                    store_obj.pos = pos;
                    store_obj.cell_id = target_id;
                    to_relocate.push((*obj_id, pos));
                    self.objs.swap_remove(i);
                }
                ObjectState::Removed => {
                    objects.remove(*obj_id);
                    self.objs.swap_remove(i);
                }
                ObjectState::Unchanged => i += 1,
            }
        }
    }
}
