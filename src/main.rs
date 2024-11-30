mod board;
mod collection;
mod helper_functions;
mod square;

use crate::board::*;

fn main() {
    let mut temp_board = Board::default();
    println!("Board Before Import:\n{}", temp_board);

    temp_board.inget_sdk_file("./sdkFiles/only_one_possible_in_col.sdk");

    println!("Board After Import:\n{}", temp_board);

    //ok, now lets do a single pass for naked singles
    let mut change_made = true;

    while change_made {
        change_made = temp_board.iterate_over_board(&Board::square_only_possible_location);
    }

    println!("Board After Single Naked Pass\n{}", temp_board);
}
