extern crate web_sys;
mod utils;
// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] //each cell is represented as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Yellow = 1,
    Red = 2,
    Empty = 0,
}

#[wasm_bindgen]
#[repr(u8)] //each cell is represented as a single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Result {
    Win,
    Draw,
    Unfinished,
    InvalidMove,
}

#[wasm_bindgen]
pub struct Board {
    width: i8,
    height: i8,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Board {
    /// Create a Connect 4 board
    pub fn new() -> Board {
        utils::set_panic_hook();
        let width = 7;
        let height = 6;
        let cells = (0..width * height).map(|_| Cell::Empty).collect();
        Board {
            width,
            height,
            cells,
        }
    }

    /// Width of the board
    pub fn width(&self) -> i8 {
        self.width
    }

    /// Height of the board
    pub fn height(&self) -> i8 {
        self.height
    }

    /// Pointer to first cell of the board
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Index to the cell, given the row and column
    fn get_index(&self, row: i8, col: i8) -> usize {
        (row * self.width + col) as usize
    }

    /// Return a vector of indexes for valid moves,
    /// given the current board
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

    /// Update board with a move (specified with row and col)
    /// return the move's result
    pub fn play(&mut self, cell: Cell, row: i8, col: i8) -> Result {
        //log!("row = {}, col = {}", row, col);
        let idx = self.get_index(row, col);
        //log!("idx = {}", idx);
        let possible_moves = self.valid_moves();
        if possible_moves.contains(&idx) {
            self.cells[idx] = cell;
            self.game_status(row, col, possible_moves)
        } else {
            Result::InvalidMove
        }
    }

    /// Returns the game outcome given a move (specified with row and col)
    /// Check whether the move contributes to a 4-cells row, column or diagonal
    fn game_status(&self, row: i8, col: i8, moves: Vec<usize>) -> Result {
        if self.four_v(row, col)
            || self.four_h(row, col)
            || self.four_dl(row, col)
            || self.four_dr(row, col)
        {
            Result::Win
        } else if moves.len() == 0 {
            Result::Draw
        } else {
            Result::Unfinished
        }
    }

    /// Check whether the move contributes to a 4-cells row
    fn four_h(&self, row: i8, col: i8) -> bool {
        let p = self.cells[self.get_index(row, col)];
        //log!("player = {:?}, row = {}, col = {}", p, row, col);
        let mut c = 0;
        let mut j = col - 1;
        while j >= 0 && self.cells[self.get_index(row, j)] == p {
            c += 1;
            //log!("c = {}, j = {}", c, j);
            j -= 1;
        }
        j = col;
        while j < self.width() && self.cells[self.get_index(row, j)] == p {
            c += 1;
            //log!("c = {}, j = {}", c, j);
            j += 1;
        }
        c >= 4
    }

    /// Check whether the move contributes to a 4-cells column
    fn four_v(&self, row: i8, col: i8) -> bool {
        let p = self.cells[self.get_index(row, col)];
        //log!("player = {:?}, row = {}, col = {}", p, row, col);
        let mut c = 0;
        let mut i = row - 1;
        while i >= 0 && self.cells[self.get_index(i, col)] == p {
            c += 1;
            //log!("c = {}, i = {}", c, i);
            i -= 1;
        }
        i = row;
        while i < self.height() && self.cells[self.get_index(i, col)] == p {
            c += 1;
            //log!("c = {}, i = {}", c, i);
            i += 1;
        }
        c >= 4
    }

    /// Check whether the move contributes to a 4-cells diagonal
    /// left going up to right
    fn four_dl(&self, row: i8, col: i8) -> bool {
        let p = self.cells[self.get_index(row, col)];
        //log!("player = {:?}, row = {}, col = {}", p, row, col);
        let mut c = 0;
        let mut j = col - 1;
        let mut i = row + 1;
        //log!(" c = {}, i = {}, j = {}", c, i, j);
        while j >= 0 && i < self.height() && self.cells[self.get_index(i, j)] == p {
            c += 1;
            i += 1;
            j -= 1;
            //log!("1) c = {}, i = {}, j = {}", c, i, j);
        }
        i = row;
        j = col;
        //log!(" c = {}, i = {}, j = {}", c, i, j);
        while j < self.width() && i >= 0 && self.cells[self.get_index(i, j)] == p {
            c += 1;
            i -= 1;
            j += 1;
            //log!("2) c = {}, i = {}, j = {}", c, i, j);
        }
        c >= 4
    }

    /// Check whether the move contributes to a 4-cells diagonal
    /// left going down to right
    fn four_dr(&self, row: i8, col: i8) -> bool {
        let p = self.cells[self.get_index(row, col)];
        //log!("player = {:?}, row = {}, col = {}", p, row, col);
        let mut c = 0;
        let mut j = col - 1;
        let mut i = row - 1;
        //log!(" c = {}, i = {}, j = {}", c, i, j);
        while j >= 0 && i >= 0 && self.cells[self.get_index(i, j)] == p {
            c += 1;
            i -= 1;
            j -= 1;
            //log!("1) c = {}, i = {}, j = {}", c, i, j);
        }
        i = row;
        j = col;
        // log!(" c = {}, i = {}, j = {}", c, i, j);

        while j < self.width() && i < self.height() && self.cells[self.get_index(i, j)] == p {
            c += 1;
            i += 1;
            j += 1;
            // log!("2) c = {}, i = {}, j = {}", c, i, j);
        }
        c >= 4
    }
}
