use glam::Vec2;
use legion::Entity;
type Cell = Vec<(Vec2, Entity)>;

pub struct DenseGrid {
    n_rows: usize,
    n_cols: usize,
    cell_size: f32,
    ll: Vec2,
    cells: Vec<Cell>,
}

impl DenseGrid {
    pub fn new(cell_size: f32, min: Vec2, max: Vec2) -> Self {
        let (n_rows, n_cols) = ((max - min) / cell_size).ceil().as_uvec2().into();

        Self {
            n_rows: n_rows as _,
            n_cols: n_rows as _,
            cell_size,
            ll: min.abs(),
            cells: (0..(n_rows * n_cols)).map(|_| Cell::default()).collect(),
        }
    }

    pub fn insert(&mut self, pos: Vec2, entity: Entity) {
        let ind = self.flat_ind(pos);
        self.cells[ind].push((pos, entity));
    }

    fn flat_ind(&self, pos: Vec2) -> usize {
        let r = ((pos.y + self.ll.y) / self.cell_size) as usize;
        let c = ((pos.x + self.ll.x) / self.cell_size) as usize;
        r * self.n_cols + c
    }

    pub fn clear(&mut self) {
        self.cells.iter_mut().for_each(|cell| cell.clear());
    }

    pub fn query(&self, pos: Vec2, radius: f32) -> Vec<Entity> {
        let min = ((pos - radius + self.ll) / self.cell_size).floor();
        let max = ((pos + radius + self.ll) / self.cell_size).floor();

        let minc = min.x as usize;
        let maxc = (max.x as usize).min(self.n_cols - 1);
        let minr = min.y as usize;
        let maxr = (max.y as usize).min(self.n_rows - 1);

        let radius2 = radius.powi(2);
        let mut hits = vec![];
        for r in minr..=maxr {
            for c in minc..=maxc {
                unsafe {
                    self.cells
                        .get_unchecked(r * self.n_cols + c)
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
