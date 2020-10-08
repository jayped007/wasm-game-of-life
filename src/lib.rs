// lib.rs -- RUST wasm interface for Conways game of life

mod utils;

use quad_rand;

use js_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
//  allows use of log! macro ==> e.g.
//    log!("cell[{}, {}] is initially {:?} and has {} neighbors",
//         row, col, cell, neighbors);
//    log!("    it becomes {:?}", next_cell);
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

// Timer generic for using web_sys::console::time and timeEnd.
//   Use new() constructor to call time and
//    use drop(&mut self) to call timeEnd.
//   So function wrapped with Timer will automatically be timed.
//   Then let _timer = Timer::new("Universe::tick");
//     will cause every call to tick() to be timed and logged on console

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

// Define a cell for the 'Universe', each 1 byte
//   use repr(u8) to ensure 1 byte unsigned values
//
//   NOTE: Define Dead value as zero and alive as one allow simple summing
//   to determine how many live cells.
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
    }

    fn set_cell(&mut self, cell_state: Cell) {
        //log!("set_cell ({:?})", cell_state);
        *self = cell_state;
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InitialPattern {
    Complex1 = 0,
    Random5050 = 1
}

// Define the 'Universe', a 1D array of Cell values (byte values, 0 or 1 per Cell def)
//   Give the width of the universe, each row of the universe is the next set
//   of 'width' cells, starting with the first row from indexes 0:<width>
#[wasm_bindgen]
pub struct Universe {
    width:  u32,  // width of each row
    height: u32,  // number of rows
    cells:  Vec<Cell>,  // width*height cells, each one byte
    prevcells: Vec<Cell>, // cells from previous tick
    mousedown: bool // set when shift-click event, so that associated click ignored
}

// methods for Universe, but not exposed to JS
impl Universe
{
    // get_index - Return 1D array index of Cell at position (row,column) in Universe
    fn get_index(&self, row: u32, column: u32) -> usize
    {
        (row * self.width + column) as usize
    }

    // Count live neighbors of cell at (row, column)
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8
    {
        // avoid modulus, division slows us down as seen in profiling
        let up = if row == 0 { self.height - 1 } else { row - 1 };
        let down = if row == self.height - 1 { 0 } else { row + 1 };
        let left = if col == 0 { self.width - 1 } else { col - 1 };
        let right = if col == self.width - 1 { 0 } else { col + 1 };

        let neighbors =
          if self.cells[self.get_index(up,left)] == Cell::Alive { 1 } else { 0 }
        + if self.cells[self.get_index(up,col)] == Cell::Alive { 1 } else { 0 }
        + if self.cells[self.get_index(up,right)] == Cell::Alive { 1 } else { 0 }
        + if self.cells[self.get_index(row,left)] == Cell::Alive { 1 } else { 0 }
        + if self.cells[self.get_index(row,right)] == Cell::Alive { 1 } else { 0 }
        + if self.cells[self.get_index(down,left)] == Cell::Alive { 1 } else { 0 }
        + if self.cells[self.get_index(down,col)] == Cell::Alive { 1 } else { 0 }
        + if self.cells[self.get_index(down,right)] == Cell::Alive { 1 } else { 0 };

        neighbors
    }   

}

// standalone method, not part of Universe directly

fn generate_cells(width: u32, height: u32, _pattern: InitialPattern) -> Vec<Cell> {

    // expression generating Vec<Cell>
    let cells = (0..width * height).map(|_i|
    {
        //if pattern == InitialPattern::Complex1 {
        //   // hardcode-pattern, depends on 8x8 definition
        //   if i % 2 == 0 || i % 7 == 0 {
        //     Cell::Alive
        //   } else {
        //     Cell::Dead
        //   }
        // } else { // InitialPattern::Random5050
            if quad_rand::gen_range(0, 20) == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        // }

    }).collect();

    cells
}

fn invert_cells(cells: &Vec<Cell>) -> Vec<Cell> {
    let count = cells.len();

    let inverted_cells = (0..count).map(|i|
    {
        if cells[i] == Cell::Alive { Cell::Dead } else { Cell::Alive }
    }).collect();

    inverted_cells
}

// Public methods, exposed to JS
#[wasm_bindgen]
impl Universe
{
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    // set_width -- set width of Universe, set all cells to Dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells =
            (0..width * self.height)
            .map(|_i| Cell::Dead).collect();
    }

    // Set the height of the Universe, set all cells to Dead state
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells =
            (0..self.width * height)
            .map(|_i| Cell::Dead).collect();
    }

    pub fn get_cell_index(&self, row: u32, column: u32) -> u32
    {
        row * self.width + column
    }

    // return pointer to 1D array of byte Cell values to JS
    //  NOTE: *const Cell syntax
    //     => pointer to non-mutable array???
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn prevcells(&self) -> *const Cell {
        self.prevcells.as_ptr()
    }
    
    pub fn tick(&mut self)
    {
        let _timer = Timer::new("Universe::tick"); // times the method, timing in browser console
          // NOTE: timing ended when _timer falls out of scope at end of method

        let mut next = self.cells.clone(); // copy of current cells, modify ==> next state
        self.prevcells = next.clone(); // previous cell values

        // Determine next state of Universe by applying conways' 4 rules
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx]; // Cell::Alive (1), or Dead (0)
                let neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, neighbors)
                {
                    // Rule 1: any live cell with < 2 live neighbors dies, (loneliness)
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: any live cell with 2 to 3 live neighbors continues to live (stable)
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: any live cell with > 3 live neighbors dies (overpopulation)
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: any dead cel with = 3 live neighbors comes alive (reproduction)
                    (Cell::Dead, 3) => Cell::Alive,
                    // Otherwise -- no change
                    (otherwise, _) => otherwise
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next; // next state for Universe determined
    }

    // toggle cell (row, column)
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn set_cell_value(&mut self, row: u32, column: u32, valu: Cell) {
        let idx = self.get_index(row, column);
        self.cells[idx].set_cell(valu);
    }

    // allow JS to determine if mousedown event occurring (shift-click)
    pub fn is_mousedown(&self) -> bool {
        return self.mousedown;
    }

    // allow JS to reset the mousedown value
    pub fn set_mousedown_value(&mut self, valu: bool) {
        self.mousedown = valu;
    }

    // Constructor, initialize the universe to hard-coded pattern
    pub fn new() -> Universe
    {
        utils::set_panic_hook(); // panic will show up in JS console, vs 'unreachable' message

        let now = js_sys::Date::now();
        let now_date = js_sys::Date::new(&JsValue::from_f64(now));

        let ms_u64: u64 = now_date.get_milliseconds() as u64;
        quad_rand::srand(ms_u64); // u64

        let width = 128; // was 64
        let height = 128;

        // Randomly decide whether to use Complex1 or Random5050
        let _pattern: InitialPattern =
          if quad_rand::gen_range(0, 2) == 0 {
            InitialPattern::Complex1
          } else {
            InitialPattern::Random5050
          };

        let pattern = InitialPattern::Random5050;
        let cells = generate_cells(width, height, pattern);
        let prevcells = invert_cells(&cells);
        let mousedown = false;

        Universe
        {
            width,
            height,
            cells,
            prevcells,
            mousedown
        }

    }

    pub fn reset_board(&mut self, pattern: InitialPattern) {
        log!("reset_board() : {:?}", pattern);
        let width = self.width();
        let height = self.height();
        self.prevcells = self.cells.clone(); // current grid, needed for correct redraw
        self.cells = generate_cells(width, height, pattern);
    }

}

// impl Universe block w/o wasm_bindgen attribute
// Needed for testing -- don't expose to our JS.
// Rust-generated WebAsm functions cannot return borrowed references.
// NOTE/SUGGEST: Try compiling the Rust-generated WebAsm with
//       the wasm_bindgen attribute and examine errors.
// NOTE: get_cells returns borrowed reference &self.cells

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set specific cells in a universe to Alive, give slice of (row,col) Tuples.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
            // NOTE: can't use self.cells[ self.get_index(row,col) ] = Cell::Alive
            //  claims immutable borrow on self.get_index call and
            //    mutable borrow later used here.  (I don't follow personally.)
        }
    }

}
