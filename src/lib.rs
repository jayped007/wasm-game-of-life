mod utils;

use quad_rand;

use js_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    cells:  Vec<Cell>  // width*height cells, each one byte
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
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8
    {
        let mut neighbors = 0;
        // Examine cells surrounging (row, column).  This is done by looking
        // at rows and columns at relative positions -1,0,0 from (row,colum).
        // NOTE: No special processing for 'last row' or 'last column'?
        //       modulus wraps the value to the 'other side of the universe'?
        for delta_row in [self.height -1, 0, 1].iter().cloned()
        {
            for delta_col in [self.width -1, 0, 1].iter().cloned()
            {
                if delta_row == 0 && delta_col == 0 {
                    continue; // ignore self
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let cell_idx = self.get_index(neighbor_row, neighbor_col);
                if self.cells[cell_idx] == Cell::Alive {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }
    
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

    // return pointer to 1D array of byte Cell values to JS
    //  NOTE: *const Cell syntax
    //     => pointer to non-mutable array???
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
    
    pub fn tick(&mut self)
    {
        let mut next = self.cells.clone(); // copy of current cells, modify ==> next state

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

    // Constructor, initialize the universe to hard-coded pattern
    pub fn new() -> Universe
    {
        let now = js_sys::Date::now();
        let now_date = js_sys::Date::new(&JsValue::from_f64(now));

        let ms_u64: u64 = now_date.get_milliseconds() as u64;
        quad_rand::srand(ms_u64); // u64

        let width = 64;
        let height = 64;

        // Randomly decide whether to use Complex1 or Random5050
        let _pattern: InitialPattern =
          if quad_rand::gen_range(0, 2) == 0 {
            InitialPattern::Complex1
          } else {
            InitialPattern::Random5050
          };

        // hardcoded pattern, depends on 8x8 definition
        //   use closure over the 1D array with zero-rel index i
        let cells = (0..width * height).map(|_i|
        {
            //if pattern == InitialPattern::Complex1 {
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

        })
        .collect();

        Universe
        {
            width,
            height,
            cells,
        }

    }

}

// implement Display trait for Universe
//   used by to_string() method used later in render() method
//   Algo:
//     Write 'height' lines, each line representing the next row in Universe
//     for each column in a row show closed square box if live else an open square box.
//   Could add following pub function to Universe
//     pub fn render(&self) -> String {
//       self.to_string()
//     }

// use std::fmt;

// impl fmt::Display for Universe
// {

//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for line in self.cells.as_slice().chunks(self.width as usize) {
//             for &cell in line {
//                 write!(f, "{}", (if cell == Cell::Dead { '◻' } else { '◼' }))?;
//             }
//             write!(f, "\n")?;
//         }
//         Ok(())
//     }

// }
