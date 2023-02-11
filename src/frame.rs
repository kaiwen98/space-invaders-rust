use std::cmp::min;
use std::cmp::max;

use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn get_safe_coords(x: i32, y: i32) -> (usize, usize) {
   (
        min(max(x, 0), (NUM_COLS - 1) as i32) as usize,
        min(max(y, 0), (NUM_ROWS - 1) as i32) as usize
   )
} 

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame) {
    }
}
