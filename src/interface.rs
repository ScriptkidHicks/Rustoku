use std::{fs::{self, File}, io::{stdin, Write}, path::Path};

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
            if ingest_sdk_file(board, trimmed_path) {
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
                save_sudoku_game(board);
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


fn save_sudoku_game(board: &Board) {
    loop {
        println!("Please enter a name of the file you would like to save the game to, or 'Exit' to exit:");
        let mut indication = String::new();

        stdin()
            .read_line(&mut indication)
            .expect("Failed to read line");

        let trimmed_indication = indication.trim();

        if trimmed_indication.to_lowercase() == "exit" {
            break;
        }

        if path_exists(trimmed_indication) {
            println!(
                "Hey, that file already exists! I can't have you deleting files that already exist!"
            );
        } else {
            save_sdk_file(board, trimmed_indication);
            break;
        }
    }
}

fn save_sdk_file(board: &Board, path: &str) {
    let mut sdk_file = File::create(path).expect("creation failed");
    sdk_file.write(board.generate_string_of_self().as_bytes()).expect("Write failed!");

    println!("Sudoku file saved successfully!");
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

pub fn ingest_sdk_file(board: &mut Board, file_path: &str) -> bool {
    match path_exists(file_path) {
        true => match digest_filepath_to_string(file_path) {
            Some(ingested_string) => {
                let string_parts = ingested_string.split('\n').collect::<Vec<&str>>();
                if string_parts.len() != 10 {
                    println!("It appears there was an incorrect number of lines in the ingested file");
                    return false;
                }

                //now lets parse each one into a row. We get a convenient Usize out of the index
                for (row_index, string_part) in string_parts.iter().enumerate() {
                    let a = string_parts.len() == 0;
                    let b = string_parts.len() == 0;
                    if a && b {
                        println!("One of the lines appears to be formatted incorrectly {} with a and b: {} {}. Clearing the board.", string_part.len(), a, b);
                        board.clear_squares();
                        return false;
                    }

                    for (col_index, char) in string_part.chars().enumerate() {
                        if char.is_numeric() {
                            match char.to_digit(10) {
                                Some(digit) => {
                                    board.set_square(row_index, col_index, digit);
                                }
                                None => {
                                    println!("oops! looks like that digit couldn't be processed correctly. Clearing the board.");
                                    board.clear_squares();
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
            None => {
                println!("Failed to digest that file into a readable string");
                return false;
            }
        },
        false => {
            println!("The path: '{}' does not appear to exist", file_path);
            return false;
        }
    }
    return true;
}