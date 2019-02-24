use crate::lib::Color;

#[derive(Copy, Clone)]
pub(crate) struct Cell {
    pub color: Color,
    pub dx: f64,
    pub dy: f64,
    pub code: u8,
}

#[derive(Clone)]
pub(crate) struct Layer {
    pub cells: Vec<Vec<Cell>>,
    size: usize,
    pub len: u8,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            color: Color(0, 0, 0, 255),
            dx: 0.0,
            dy: 0.0,
            code: 0,
        }
    }
}

impl Layer {
    pub fn new(size: usize) -> Layer {
        Layer {
            cells: vec![vec![Cell::new(); size]],
            size: size,
            len: 1,
        }
    }

    pub fn add_layer(&mut self) {
        self.cells.push(vec![Cell::new(); self.size]);
        self.len += 1;
    }

    pub fn clear_layer(&mut self, layer: u8) {
        for i in 0..self.size {
            self.cells[layer as usize][i as usize].code = 0;
        }
    }
}
