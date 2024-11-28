use std::{fmt, fs, path::Path};

use crate::collection::*;
use crate::helper_functions::*;

pub struct Board {
    rows: [Collection; 9],
    columns: [Collection; 9],
    cubes: [Collection; 9], //cubes are iterated left to right, top to bottom
    unsolved_squares: u32,
}

impl Board {
    pub fn default() -> Board {
        Board {
            rows: [Collection::default(); 9],
            columns: [Collection::default(); 9],
            cubes: [Collection::default(); 9],
            unsolved_squares: 81,
        }
    }

    pub fn set_square(&mut self, row_index: usize, col_index: usize, value: u32) {
        // we have to do a little bit of indexing math to get the correct cube location in our array of cubes
        let (cube_index, inner_cube_index) = row_and_col_to_cube_location(row_index, col_index);
        self.rows[row_index].set_square(col_index, value);
        self.columns[col_index].set_square(row_index, value);
        self.cubes[cube_index].set_square(inner_cube_index, value);
        self.unsolved_squares -= 1;
    }

    pub fn clear_squares(&mut self) {
        for row_index in 0..9 {
            for col_index in 0..9 {
                let (cube_index, inner_cube_index) =
                    row_and_col_to_cube_location(row_index, col_index);
                self.rows[row_index].set_square(col_index, 0);
                self.columns[col_index].set_square(row_index, 0);
                self.cubes[cube_index].set_square(inner_cube_index, 0);
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
        let (cube_major, cube_minor) = row_and_col_to_cube_location(row_index, col_index);
        let row_possibles = self.rows[row_index].get_possible_numbers(col_index);
        let col_possibles = self.columns[col_index].get_possible_numbers(row_index);
        let cube_possibles = self.cubes[cube_major].get_possible_numbers(cube_minor);
        let possibles = collapse_three_vectors(row_possibles, col_possibles, cube_possibles);
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, row) in self.rows.iter().enumerate() {
            println!("{}", row);
            if index == 2 || index == 5 {
                println!("---------   ---------   ---------");
            }
        }
        Ok(())
    }
}
