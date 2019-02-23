use crate::lib::Color;

#[derive(Copy, Clone)]
pub(crate) struct Cell {
    pub color: Color,
    pub dx: f32,
    pub dy: f32,
    pub code: char,
}

#[derive(Clone)]
pub(crate) struct Layer {
    pub cells: Vec<Cell>,
}

impl Layer {
    pub fn new(size: usize) -> Layer {
        Layer {
            cells: vec![Cell::new(); size],
        }
    }
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            color: Color(0, 0, 0, 255),
            dx: 0.0,
            dy: 0.0,
            code: '\0',
        }
    }
}
