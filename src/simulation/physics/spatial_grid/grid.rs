use glam::Vec2;
use legion::Entity;
use slotmap::{new_key_type, SlotMap};

use super::{
    cell::{CellObject, GridCell},
    storage::{cell_range, CellIdx, DenseStorage},
};

pub type GridObjects = SlotMap<GridHandle, StoreObject>;

new_key_type! {
    /// This handle is used to modify the associated object or to update its position.
    /// It is returned by the _insert_ method of a Grid.
    pub struct GridHandle;
}

/// State of an object, maintain() updates the internals of the grid and resets this to Unchanged
#[derive(Clone, Copy, Debug)]
pub enum ObjectState {
    Unchanged,
    NewPos(Vec2),
    Relocate(Vec2, CellIdx),
    Removed,
}

/// The actual object stored in the store
#[derive(Clone, Copy)]
pub struct StoreObject {
    /// User-defined object to be associated with a value
    obj: Entity,
    pub state: ObjectState,
    pub pos: Vec2,
    pub cell_id: CellIdx,
}

#[derive(Clone)]
pub struct DenseGrid {
    storage: DenseStorage<GridCell>,
    objects: GridObjects,
    // Cache maintain vec to avoid allocating every time maintain is called
    to_relocate: Vec<CellObject>,
}

#[allow(dead_code)]
impl DenseGrid {
    /// Creates an empty grid.   
    /// The cell size should be about the same magnitude as your queries size.
    pub fn new(cell_size: i32) -> Self {
        Self::with_storage(DenseStorage::new(cell_size))
    }

    /// Creates an empty grid.   
    /// The cell size should be about the same magnitude as your queries size.
    pub fn with_storage(st: DenseStorage<GridCell>) -> Self {
        Self {
            storage: st,
            objects: SlotMap::with_key(),
            to_relocate: vec![],
        }
    }

    /// Inserts a new object with a position and an associated object
    /// Returns the unique and stable handle to be used with get_obj
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    /// let mut g: DenseGrid<()> = DenseGrid::new(10);
    /// let h = g.insert([5.0, 3.0].into(), ());
    /// ```
    pub fn insert(&mut self, pos: Vec2, obj: Entity) -> GridHandle {
        let (cell_id, cell) = self.storage.cell_mut(pos);
        let handle = self.objects.insert(StoreObject {
            obj,
            state: ObjectState::Unchanged,
            pos,
            cell_id,
        });
        cell.objs.push((handle, pos));
        handle
    }

    /// Lazily sets the position of an object (if it is not marked for deletion).
    /// This won't be taken into account until maintain() is called.  
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    /// let mut g: DenseGrid<()> = DenseGrid::new(10);
    /// let h = g.insert([5.0, 3.0].into(), ());
    /// g.set_position(h, [3.0, 3.0].into());
    /// ```
    pub fn set_position(&mut self, handle: GridHandle, pos: Vec2) {
        let obj = match self.objects.get_mut(handle) {
            Some(x) => x,
            None => {
                debug_assert!(false, "Object not in grid anymore");
                return;
            }
        };

        if matches!(obj.state, ObjectState::Removed) {
            return;
        }

        let target_id = self.storage.cell_id(pos);
        obj.state = if target_id == obj.cell_id {
            ObjectState::NewPos(pos)
        } else {
            ObjectState::Relocate(pos, target_id)
        };

        self.storage.cell_mut_unchecked(obj.cell_id).dirty = true;
    }

    /// Lazily removes an object from the grid.
    /// This won't be taken into account until maintain() is called.  
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    /// let mut g: DenseGrid<()> = DenseGrid::new(10);
    /// let h = g.insert([5.0, 3.0].into(), ());
    /// g.remove(h);
    /// ```
    pub fn remove(&mut self, handle: GridHandle) -> Option<Entity> {
        let obj = self.objects.get_mut(handle)?;

        obj.state = ObjectState::Removed;
        self.storage.cell_mut_unchecked(obj.cell_id).dirty = true;

        Some(obj.obj)
    }

    /// Maintains the world, updating all the positions (and moving them to corresponding cells)
    /// and removing necessary objects and empty cells.
    /// Runs in linear time O(N) where N is the number of objects.
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    /// let mut g: DenseGrid<()> = DenseGrid::new(10);
    /// let h = g.insert([5.0, 3.0].into(), ());
    /// g.remove(h);
    ///
    /// assert!(g.get(h).is_some());
    /// g.maintain();
    /// assert!(g.get(h).is_none());
    /// ```
    pub fn maintain(&mut self) {
        let Self {
            storage,
            objects,
            to_relocate,
            ..
        } = self;

        storage.modify(|cell| {
            cell.maintain(objects, to_relocate);
            cell.objs.is_empty()
        });

        for (handle, pos) in to_relocate.drain(..) {
            storage.cell_mut(pos).1.objs.push((handle, pos));
        }
    }

    /// Iterate over all handles
    pub fn handles(&self) -> impl Iterator<Item = GridHandle> + '_ {
        self.objects.keys()
    }

    /// Iterate over all objects
    pub fn objects(&self) -> impl Iterator<Item = &Entity> + '_ {
        self.objects.values().map(|x| &x.obj)
    }

    /// Returns a reference to the associated object and its position, using the handle.  
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    /// let mut g: DenseGrid<i32> = DenseGrid::new(10);
    /// let h = g.insert([5.0, 3.0].into(), 42);
    /// assert_eq!(g.get(h), Some(([5.0, 3.0].into(), &42)));
    /// ```
    pub fn get(&self, id: GridHandle) -> Option<(Vec2, &Entity)> {
        self.objects.get(id).map(|x| (x.pos, &x.obj))
    }

    /// Returns a mutable reference to the associated object and its position, using the handle.  
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    /// let mut g: DenseGrid<i32> = DenseGrid::new(10);
    /// let h = g.insert([5.0, 3.0].into(), 42);
    /// *g.get_mut(h).unwrap().1 = 56;
    /// assert_eq!(g.get(h).unwrap().1, &56);
    /// ```    
    pub fn get_mut(&mut self, id: GridHandle) -> Option<(Vec2, &mut Entity)> {
        self.objects.get_mut(id).map(|x| (x.pos, &mut x.obj))
    }

    /// The underlying storage
    pub fn storage(&self) -> &DenseStorage<GridCell> {
        &self.storage
    }

    /// Queries for all objects around a position within a certain radius.
    /// Try to keep the radius asked and the cell size of similar magnitude for better performance.
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    ///
    /// let mut g: DenseGrid<()> = DenseGrid::new(10);
    /// let a = g.insert([0.0, 0.0].into(), ());
    ///
    /// let around: Vec<_> = g.query_around([2.0, 2.0].into(), 5.0).map(|(id, _pos)| id).collect();
    ///
    /// assert_eq!(vec![a], around);
    /// ```
    pub fn query_around(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = CellObject> + '_ {
        let ll = pos - radius; // lower left
        let ur = pos + radius; // upper right

        let radius2 = radius * radius;
        self.query_raw(ll, ur)
            .filter(move |(_, pos_obj)| pos_obj.distance_squared(pos) < radius2)
    }

    /// Queries for all objects in the cells intersecting an axis-aligned rectangle defined by lower left (ll) and upper right (ur)
    /// Try to keep the rect's width/height of similar magnitudes to the cell size for better performance.
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    ///
    /// let mut g: DenseGrid<()> = DenseGrid::new(10);
    /// let a = g.insert([0.0, 0.0].into(), ());
    /// let b = g.insert([5.0, 5.0].into(), ());
    ///
    /// let around: Vec<_> = g.query_raw([-1.0, -1.0].into(), [1.0, 1.0].into()).map(|(id, _pos)| id).collect();
    ///
    /// assert_eq!(vec![a, b], around);
    /// ```
    pub fn query_raw(&self, ll: Vec2, ur: Vec2) -> impl Iterator<Item = CellObject> + '_ {
        let ll_id = self.storage.cell_id(ll);
        let ur_id = self.storage.cell_id(ur);

        cell_range(ll_id, ur_id)
            .flat_map(move |id| self.storage.cell(id))
            .flat_map(|x| x.objs.iter().copied())
    }

    /// Allows to look directly at what's in a cell covering a specific position.
    ///
    /// # Example
    /// ```rust
    /// use flat_spatial::DenseGrid;
    ///
    /// let mut g: DenseGrid<()> = DenseGrid::new(10);
    /// let a = g.insert([2.0, 2.0].into(), ());
    ///
    /// let around = g.get_cell([1.0, 1.0].into()).collect::<Vec<_>>();
    ///
    /// assert_eq!(vec![(a, [2.0, 2.0].into())], around);
    /// ```
    pub fn get_cell(&mut self, pos: Vec2) -> impl Iterator<Item = CellObject> + '_ {
        self.storage
            .cell(self.storage.cell_id(pos))
            .into_iter()
            .flat_map(|x| x.objs.iter().copied())
    }

    /// Returns the number of objects currently available
    /// (removals that were not confirmed with maintain() are still counted)
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Checks if the grid contains objects or not
    /// (removals that were not confirmed with maintain() are still counted)
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
}
