use itertools::Itertools;
use legion::Entity;
use rayon::prelude::*;
use ultraviolet::Vec2;

use super::cell::Cell;

const U32_SIZE: u32 = (std::mem::size_of::<u32>() as u32) * 8;

fn log_2(x: u32) -> u32 {
    debug_assert!(x > 0);
    U32_SIZE - x.leading_zeros() - 1
}

pub struct DenseGrid {
    /// log2(ncells_side)
    log2_side: u32,
    /// log2(cell_size)
    log2_cell: u32,

    cells: Vec<Cell>,
}

impl DenseGrid {
    pub fn new(cell_size: u32, side_len: u32) -> Self {
        assert!(side_len.is_power_of_two());
        assert!(cell_size.is_power_of_two());
        let ncells_side = side_len / cell_size;
        Self {
            log2_side: log_2(ncells_side),
            log2_cell: log_2(cell_size),
            cells: (0..(ncells_side * ncells_side)).map(|_| Cell::default()).collect(),
        }
    }

    pub fn insert(&self, pos: Vec2, entity: Entity) {
        let ind = self.flat_ind(pos);
        let cell = self.cells.get(ind).unwrap();
        cell.insert(pos, entity);
    }

    #[inline]
    pub fn flat_ind(&self, pos: Vec2) -> usize {
        let x = (pos.x as u32) >> self.log2_cell;
        let y = (pos.y as u32) >> self.log2_cell;
        ((y << self.log2_side) | x) as usize
    }

    pub fn clear(&mut self) {
        self.cells.par_iter().for_each(|cell| cell.clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32, ignore: Entity) -> Vec<Entity> {
        let radius2 = radius.powi(2);
        let mut hits = Vec::with_capacity(2);

        for ind in self.cell_range(pos, radius) {
            if let Some(cell) = self.cells.get(ind as usize) {
                // We know this is at a read only stage. Safe to disregard lock
                for (other, id) in cell.unlock_unsafe() {
                    if (*id != ignore) & ((pos - *other).mag_sq() < radius2) {
                        hits.push(*id);
                    }
                }
            }
        }

        hits
    }

    pub fn cell_range(&self, pos: Vec2, radius: f32) -> impl Iterator<Item = u32> {
        let x1 = ((pos.x - radius) as u32) >> self.log2_cell;
        let y1 = ((pos.y - radius) as u32) >> self.log2_cell;
        let x2 = ((pos.x + radius) as u32) >> self.log2_cell;
        let y2 = ((pos.y + radius) as u32) >> self.log2_cell;

        let shift = self.log2_side;

        (x1..=x2).cartesian_product(y1..=y2).map(move |(x, y)| (y << shift) | x)
    }
}
