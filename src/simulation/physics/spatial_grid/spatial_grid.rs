use glam::Vec2;
use legion::Entity;
type Cell = Vec<(Vec2, Entity)>;

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: u32) -> u32 {
    assert!(x > 0);
    num_bits::<u32>() as u32 - x.leading_zeros() - 1
}

pub struct DenseGrid {
    /// Side length, must be power of 2
    side_len: u32,
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
            side_len,
            log2_side: log_2(ncells_side),
            log2_cell: log_2(cell_size),
            cells: (0..(ncells_side * ncells_side)).map(|_| Cell::default()).collect(),
        }
    }

    pub fn insert(&mut self, pos: Vec2, entity: Entity) {
        let ind = self.flat_ind(pos);
        self.cells[ind].push((pos, entity));
    }

    fn flat_ind(&self, pos: Vec2) -> usize {
        let x = (pos.x.floor() as u32) >> self.log2_cell;
        let y = (pos.y.floor() as u32) >> self.log2_cell;
        ((y << self.log2_side) | x) as usize
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|cell| cell.clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32) -> Vec<Entity> {
        let x1 = ((pos.x - radius).max(0.0).floor() as u32) >> self.log2_cell;
        let y1 = ((pos.y - radius).max(0.0).floor() as u32) >> self.log2_cell;
        let x2 = ((pos.x + radius).min((self.side_len - 1) as f32).floor() as u32) >> self.log2_cell;
        let y2 = ((pos.y + radius).min((self.side_len - 1) as f32).floor() as u32) >> self.log2_cell;

        let radius2 = radius.powi(2);
        let mut hits = vec![];
        for x in x1..=x2 {
            for y in y1..=y2 {
                unsafe {
                    self.cells
                        .get_unchecked(((y << self.log2_side) | x) as usize)
                        .iter()
                        .filter_map(|(other, id)| match pos.distance_squared(*other) < radius2 {
                            true => Some(id),
                            false => None,
                        })
                        .for_each(|id| hits.push(*id))
                }
            }
        }

        hits
    }
}
