use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] //each cell is represented as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Yellow,
    Red,
    Empty,
}

#[wasm_bindgen]
pub struct Board {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

impl Board {}

#[wasm_bindgen]
impl Board {
    pub fn new() -> Board {
        let width = 7;
        let height = 6;
        let cells = (0..width * height).map(|_| Cell::Empty).collect();
        Board {
            width,
            height,
            cells,
        }
    }
    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}
