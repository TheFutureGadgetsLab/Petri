use glam::{vec2, Vec2};
use itertools::Itertools;
use legion::Entity;
use parking_lot::RwLock;

type Cell = Vec<(Vec2, Entity)>;

const U32_SIZE: u32 = (std::mem::size_of::<u32>() as u32) * 8;

fn log_2(x: u32) -> u32 {
    debug_assert!(x > 0);
    U32_SIZE - x.leading_zeros() - 1
}

pub struct DenseGrid {
    /// Side length, must be power of 2
    side_len: f32,
    /// log2(ncells_side)
    log2_side: u32,
    /// log2(cell_size)
    log2_cell: u32,
    /// Corner padding to add to ensure there are no out-of-bounds queries
    pad: Vec2,

    cells: Vec<RwLock<Cell>>,
}

impl DenseGrid {
    pub fn new(cell_size: u32, side_len: u32) -> Self {
        assert!(side_len.is_power_of_two());
        assert!(cell_size.is_power_of_two());
        let ncells_side = side_len / cell_size;
        Self {
            side_len: side_len as f32,
            log2_side: log_2(ncells_side),
            log2_cell: log_2(cell_size),
            pad: Vec2::new((cell_size / 2) as f32, (cell_size / 2) as f32),
            cells: (0..(ncells_side * ncells_side))
                .map(|_| RwLock::new(Cell::default()))
                .collect(),
        }
    }

    pub fn safe_bounds(&self) -> (Vec2, Vec2) {
        (self.pad, vec2(self.side_len, self.side_len) - (self.pad * 2.0))
    }

    pub fn insert(&self, pos: Vec2, entity: Entity) {
        if let Some(cell) = self.cells.get(self.flat_ind(pos)) {
            cell.write().push((pos, entity));
        }
    }

    #[inline]
    pub fn flat_ind(&self, pos: Vec2) -> usize {
        let x = ((pos.x + self.pad.x) as u32) >> self.log2_cell;
        let y = ((pos.y + self.pad.y) as u32) >> self.log2_cell;
        ((y << self.log2_side) | x) as usize
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|cell| cell.write().clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32, ignore: Entity) -> Vec<Entity> {
        let radius2 = radius.powi(2);
        let mut hits = Vec::with_capacity(4);

        for ind in self.cell_range(pos, radius) {
            if let Some(cell) = self.cells.get(ind as usize) {
                // We know this is at a read only stage. Safe to disregard lock
                let unlocked = unsafe { cell.data_ptr().as_ref().unwrap() };
                hits.extend(unlocked.iter().filter_map(|(other, id)| {
                    match (*id != ignore) & (pos.distance_squared(*other) < radius2) {
                        true => Some(*id),
                        false => None,
                    }
                }));
            }
        }

        hits
    }

    pub fn cell_range(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = u32> {
        let (tr_x, tr_y) = (pos + self.pad + radius).as_uvec2().into();
        let (bl_x, bl_y) = (pos + self.pad - radius).as_uvec2().into();
        let x1 = bl_x >> self.log2_cell;
        let y1 = bl_y >> self.log2_cell;
        let x2 = tr_x >> self.log2_cell;
        let y2 = tr_y >> self.log2_cell;
        let shift = self.log2_side;

        (x1..=x2).cartesian_product(y1..=y2).map(move |(x, y)| (y << shift) | x)
    }
}
