use std::io::stdin;

use crate::Board;

pub fn run_interface() {
    println!("Welcome to Rustoku.");

    loop {
        let mut play_board = Board::default();

        println!("Please enter a selection");
        println!("1: Import a Sudoku SDK file");
        println!("2: Exit");

        let mut selection = String::new();

        stdin()
            .read_line(&mut selection)
            .expect("Failed to read input");

        let indication_number: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It appears you entered something that wasn't a positive integer. Oops!");
                continue;
            }
        };

        match indication_number {
            1 => match query_file_for_ingestion(play_board) {
                true => {}
                false => {
                    continue;
                }
            },
            2 => {
                break;
            }
            _ => {
                println!("Oops! It looks like you entered an invalid number!");
            }
        }
    }
}

pub fn query_file_for_ingestion(board: Board) -> bool {
    let mut success = false;
    loop {
        println!("Please enter a file name for the sdk file you want to work on, or type 'Exit' to quit.");
        let mut path = String::new();

        stdin().read_line(&mut path).expect("Failed to read line");
    }

    success
}
