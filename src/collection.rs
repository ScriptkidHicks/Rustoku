use std::fmt;

use crate::square::*;

#[derive(Copy, Clone)]
pub struct Collection {
    squares: [Square; 9],
}

impl Collection {
    pub fn default() -> Collection {
        Collection {
            //We can't just do the x; 9 version because it requires clone, and we have a vector in place
            squares: [Square::default(); 9],
        }
    }

    pub fn set_square(&mut self, index: usize, value: u32) {
        self.squares[index].set_value(value);
        // be sure to remove this possiblity from other squares in the collection
        for square in self.squares.iter_mut() {
            if square.is_empty() {
                square.remove_possibility(value);
            }
        }
    }

    pub fn square_empty(&self, index: usize) -> bool {
        self.squares[index].is_empty()
    }

    pub fn get_possible_numbers(&self, index: usize) -> Vec<u32> {
        self.squares[index].get_possible_numbers()
    }
}

impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, square) in self.squares.iter().enumerate() {
            print!("{}", square);
            if (index == 2 || index == 5) {
                print!(" | ");
            }
        }
        Ok(())
    }
}