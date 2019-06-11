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
#[repr(u8)] //each cell is represented as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    Ok,
    Invalid,
}

#[wasm_bindgen]
pub struct Board {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

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

    fn get_index(&self, row: u8, column: u8) -> usize {
        (row * self.width + column) as usize
    }

    fn get_coords(&self, index: usize) -> (u8, u8) {
        let row = index / (self.width as usize);
        let col = index % (self.width as usize);
        (row as u8, col as u8)
    }

    fn valid_moves(&self) -> Vec<usize> {
        let mut moves = Vec::new();
        for col in (0..self.width).into_iter() {
            for row in (0..self.height).into_iter().rev() {
                let idx = self.get_index(row, col);
                if self.cells[idx] == Cell::Empty {
                    moves.push(idx);
                    break;
                }
            }
        }
        moves
    }

    pub fn play(&mut self, cell: Cell, index: usize) -> Move {
        if self.valid_moves().contains(&index) {
            self.cells[index] = cell;
            Move::Ok
        } else {
            Move::Invalid
        }
    }
}
