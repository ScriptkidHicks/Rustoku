mod Square; // the mod is actually square, lower case, but for some reason the rust analyzer still thinks that it's upper case. Incredibly annoying, but I can't fix it, because the analyzer won't let go of a reference that doesn't exist anymore.
mod board;
mod collection;
mod helper_functions;
mod square;

use crate::board::*;

fn main() {
    let mut temp_board = Board::default();
    println!("Board Before Import:\n{}", temp_board);

    temp_board.inget_sdk_file("./sdkFiles/1.sdk");

    println!("Board After Import:\n{}", temp_board);

    //ok, now lets do a single pass for naked singles

    temp_board.iterate_over_board(&Board::naked_single);

    println!("Board After Single Naked Pass\n{}", temp_board);
}
