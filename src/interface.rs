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

fn utilize_solution_method(board: &mut Board) {
    loop {
        let continuous: bool;
        let solution_method: fn(&mut Board, usize, usize) -> bool;
        let mut change_made = false;

        loop {
            println!(
                "Would you like the solution to iterate once, or continuously until changes cease?"
            );
            println!("1: Once");
            println!("2: Continuously");
            println!("3: Exit");
            let mut continuous_input = String::new();
            stdin()
                .read_line(&mut continuous_input)
                .expect("Failed to read line");

            let indication_number: u32 = match continuous_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!(
                        "It appears you entered something that wasn't a positive integer. Oops!"
                    );
                    continue;
                }
            };

            match indication_number {
                1 => {
                    continuous = false;
                    break;
                }
                2 => {
                    continuous = true;
                    break;
                }
                3 => {
                    return;
                }
                _ => {
                    println!("It looks like you selected a number that wasn't an option.");
                }
            }
        }

        loop {
            println!("Please select a solution function");
            println!("1: Naked Singles");
            println!("2: Hidden Singles");
            println!("3: Exit");

            let mut function_input = String::new();
            stdin()
                .read_line(&mut function_input)
                .expect("Failed to read line");

            let fun_indication: u32 = match function_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!(
                        "It appears you entered something that wasn't a positive integer. Oops!"
                    );
                    continue;
                }
            };

            match fun_indication {
                1 => {
                    solution_method = Board::naked_single;
                    break;
                }
                2 => {
                    solution_method = Board::hidden_single;
                    break;
                }
                3 => {
                    return;
                }
                _ => {
                    println!("It looks like you selected a number that wasn't an option.");
                }
            }
        }

        loop {
            let circuit_change_made = board.iterate_over_board(&solution_method);
            change_made = change_made || circuit_change_made;
            if circuit_change_made && continuous {
                continue;
            } else {
                break;
            }
        }

        if (change_made) {
            println!("A change was made to the board.");
        } else {
            println!("The board was unchanged.")
        }

        println!("{}", board);

        break;
    }
}
