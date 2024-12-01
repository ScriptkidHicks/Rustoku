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
    println!("now lets take a look at the possibilities");
    temp_board.show_me_the_possibilities();
}
