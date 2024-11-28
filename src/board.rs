use std::{fmt, fs, path::Path};

use crate::collection::*;
use crate::square::*;

pub struct Board {
    rows: [Collection; 9],
    unsolved_squares: u32,
}

impl Board {
    //a board only has rows. columns and cubes are conceptual indexes, rather than actual structures.
    //THIS IS BECAUSE RUST WON'T LET ME HAVE MUTUAL REFERENCE FOR OBVIOUS BUT ANNOYING REASONS
    pub fn default() -> Board {
        Board {
            rows: [Collection::default(); 9],
            unsolved_squares: 81,
        }
    }

    pub fn col_iter_mut(
        &mut self,
        col_index: usize,
        value: u32,
        callback: &dyn Fn(&mut Square, u32) -> (),
    ) {
        for row_index in 0..9 {
            self.rows[row_index].alter_square(col_index, value, callback);
        }
    }

    pub fn cube_iter_mut(
        &mut self,
        row_index: usize,
        col_index: usize,
        value: u32,
        callback: &dyn Fn(&mut Square, u32) -> (),
    ) {
        let row_floor = (row_index / 3) * 3;
        let col_floor = (col_index / 3) * 3;

        for internal_row_index in row_floor..(row_floor + 3) {
            for internal_col_index in col_floor..(col_floor + 3) {
                self.rows[internal_row_index].alter_square(internal_col_index, value, callback);
            }
        }
    }

    pub fn set_square(&mut self, row_index: usize, col_index: usize, value: u32) {
        //this will automatically remove the possiblity from the row, but we also need to do column and cube
        self.rows[row_index].set_square(col_index, value);
        self.col_iter_mut(col_index, value, &Square::remove_possibility);
        self.cube_iter_mut(row_index, col_index, value, &Square::remove_possibility);
        self.unsolved_squares -= 1;
    }

    pub fn clear_squares(&mut self) {
        for row_index in 0..9 {
            for col_index in 0..9 {
                self.rows[row_index].set_square(col_index, 0);
            }
        }

        self.unsolved_squares = 81;
    }

    pub fn square_empty(&self, row_index: usize, col_index: usize) -> bool {
        self.rows[row_index].square_empty(col_index)
    }

    pub fn self_solved(&self) -> bool {
        self.unsolved_squares == 0
    }

    pub fn path_exists(file_path: &str) -> bool {
        Path::new(file_path).exists()
    }

    pub fn digest_filepath_to_string(file_path: &str) -> Option<String> {
        match fs::read_to_string(file_path) {
            Ok(string_value) => Some(string_value),
            Err(_) => None,
        }
    }

    pub fn inget_sdk_file(&mut self, file_path: &str) {
        match Board::path_exists(file_path) {
            true => match Board::digest_filepath_to_string(file_path) {
                Some(ingested_string) => {
                    let string_parts = ingested_string.split('\n').collect::<Vec<&str>>();
                    if string_parts.len() != 10 {
                        println!("It appears there was an incorrect number of lines in the ingested file");
                        return;
                    }

                    //now lets parse each one into a row. We get a convenient Usize out of the index
                    for (row_index, string_part) in string_parts.iter().enumerate() {
                        let a = string_parts.len() == 0;
                        let b = string_parts.len() == 0;
                        if a && b {
                            println!("One of the lines appears to be formatted incorrectly {} with a and b: {} {}. Clearing the board.", string_part.len(), a, b);
                            self.clear_squares();
                            return;
                        }

                        for (col_index, char) in string_part.chars().enumerate() {
                            if char.is_numeric() {
                                match char.to_digit(10) {
                                    Some(digit) => {
                                        self.set_square(row_index, col_index, digit);
                                    }
                                    None => {
                                        println!("oops! looks like that digit couldn't be processed correctly. Clearing the board.");
                                        self.clear_squares();
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
                None => {
                    println!("Failed to digest that file into a readable string");
                }
            },
            false => {
                println!("The path: '{}' does not appear to exist", file_path);
            }
        }
    }

    //this function gets the intersection of possibilities for row, col, and cube at a square index
    pub fn get_possible_numbers(&self, row_index: usize, col_index: usize) -> Vec<u32> {
        let row_possibles = self.rows[row_index].get_possible_numbers(col_index);
        let possibles = row_possibles;
        if possibles.len() == 0 {
            panic!("Somehow an empty square has no possibilities. This should be impossible");
        }
        possibles
    }

    //this function iterates over the board, left to right, top to bottom, looking at each square.
    //it then calls a passed function to make changes to the board as necessary.
    pub fn iterate_over_board(
        &mut self,
        solver_function: &dyn Fn(
            &mut Board,
            usize,
            usize, /*so they can index into their collection to that square's index. It's different for each of them */
        ) -> bool,
    ) -> bool {
        //returns a bool depending on whether the board state changed under this pass
        let mut change_occured = false;

        for row_index in 0..9 {
            for col_index in 0..9 {
                if self.square_empty(row_index, col_index) {
                    //don't bother looking at squares that aren't empty.
                    change_occured = change_occured || solver_function(self, row_index, col_index);
                }
            }
        }

        change_occured
    }

    pub fn naked_single(board: &mut Board, row_index: usize, col_index: usize) -> bool {
        let mut change_occured = false;

        //our iterator guarantees that the square we're looking at is not empty, so we know we're safe to treat it as such.
        let possible_numbers = board.get_possible_numbers(row_index, col_index);
        if possible_numbers.len() == 1 {
            // oh look, only one number could possibly go there.
            match possible_numbers.last() {
                Some(singlet) => {
                    board.set_square(row_index, col_index, *singlet);
                    change_occured = true;
                }
                None => {
                    panic!("somehow the vector is of length 1, but contains no last. This is a major error.")
                }
            }
        }
        change_occured
    }
}

impl fmt::Display for Board {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        for (index, row) in self.rows.iter().enumerate() {
            println!("{}", row);
            if index == 2 || index == 5 {
                println!("---------   ---------   ---------");
            }
        }
        Ok(())
    }
}
