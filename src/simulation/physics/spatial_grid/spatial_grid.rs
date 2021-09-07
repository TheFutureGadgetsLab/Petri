use glam::{vec2, Vec2};
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

    pub fn insert(&self, pos: Vec2, entity: Entity, ind: usize) {
        self.cells[ind].write().push((pos, entity));
    }

    pub fn flat_ind(&self, pos: Vec2) -> usize {
        let x = ((pos.x + self.pad.x) as u32) >> self.log2_cell;
        let y = ((pos.y + self.pad.y) as u32) >> self.log2_cell;
        ((y << self.log2_side) | x) as usize
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|cell| cell.write().clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32, ignore: Entity) -> Vec<Entity> {
        let tr = pos + self.pad + radius;
        let bl = pos + self.pad - radius;
        let x1 = ((bl.x) as u32) >> self.log2_cell;
        let y1 = ((bl.y) as u32) >> self.log2_cell;
        let x2 = ((tr.x) as u32) >> self.log2_cell;
        let y2 = ((tr.y) as u32) >> self.log2_cell;

        let radius2 = radius.powi(2);
        let mut hits = Vec::with_capacity(4);
        for y in y1..=y2 {
            let s = y << self.log2_side;
            for x in x1..=x2 {
                // We know this is at a read only stage. Safe to disregard lock
                let cell = unsafe { self.cells[(s | x) as usize].data_ptr().as_ref().unwrap() };
                hits.extend(cell.iter().filter_map(|(other, id)| {
                    match (*id != ignore) & (pos.distance_squared(*other) < radius2) {
                        true => Some(*id),
                        false => None,
                    }
                }));
            }
        }

        hits
    }
}
