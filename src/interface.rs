use std::{io::stdin, path::Path};

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
            1 => match query_file_for_ingestion(&mut play_board) {
                true => {
                    user_solve_sudoku(&mut play_board);
                }
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

pub fn query_file_for_ingestion(board: &mut Board) -> bool {
    let mut success = false;
    loop {
        println!("Please enter a file name for the sdk file you want to work on, or type 'Exit' to quit.");
        let mut path = String::new();

        stdin().read_line(&mut path).expect("Failed to read line");

        let trimmed_path = path.trim(); //need to remove the newline that will occur on input.

        if trimmed_path.to_lowercase() == "exit" {
            break;
        }

        if Path::new(trimmed_path).exists() {
            if board.ingest_sdk_file(trimmed_path) {
                success = true;
                println!("Board imported successfully.");
                break;
            }
        } else {
            println!("It appears that file doesn't exist!");
        }
    }

    success
}

pub fn user_solve_sudoku(board: &mut Board) {
    println!("Your starting state board:\n{}", board);

    loop {
        println!("Please enter a selection:");
        println!("1: Display board possibilities");
        println!("2: Solution method");
        println!("3: Save board");
        println!("4: Exit");

        let mut selection = String::new();

        stdin()
            .read_line(&mut selection)
            .expect("Failed to read line");

        let indication_number: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It appears you entered something that wasn't a positive integer. Oops!");
                continue;
            }
        };

        match indication_number {
            1 => {
                board.show_me_the_possibilities();
            }
            2 => {
                utilize_solution_method(board);
            }
            3 => {
                break;
            }
            4 => {
                break;
            }
            _ => {}
        }
    }
}

fn utilize_solution_method(board: &mut Board) {}
