pub(crate) mod board;
pub(crate) mod element;
pub(crate) mod index;
pub(crate) mod mouse;
pub(crate) mod scanner;
pub(crate) mod screen;
pub(crate) mod solver;

use std::{
    io::{stdout, Write},
    thread::sleep,
    time::Duration,
};

use board::Step;
use image::{DynamicImage, RgbImage};
use mouse_rs::Mouse;
use scanner::Scanner;
use screenshots::Screen;
use solver::InitialBoard;

use crate::solver::SolveResult;

#[derive(Clone, Debug, PartialEq, Eq)]
enum BoardState {
    SleepSearch,
    SleepValidate(InitialBoard),
    SleepUnsolvable(InitialBoard),
    Search,
    Validate(InitialBoard),
    Ready(InitialBoard),
    Solve(Vec<Step>),
    Unsolvable(InitialBoard),
}

fn main() {
    let mouse = Mouse::new();
    let scanner = Scanner::new();

    let mut state = BoardState::Search;
    loop {
        match state {
            BoardState::SleepSearch => {
                sleep(Duration::from_secs(1));
                state = BoardState::Search;
            }
            BoardState::SleepValidate(board) => {
                print!("Making sure nothing is moving...");
                stdout().flush().unwrap();
                sleep(Duration::from_secs(2));
                state = BoardState::Validate(board);
            }
            BoardState::SleepUnsolvable(board) => {
                sleep(Duration::from_secs(1));
                state = BoardState::Unsolvable(board);
            }
            BoardState::Search => {
                let image = capture_first_screen();

                state = match scanner.scan_image(&image) {
                    Some(board) => {
                        println!("Found a valid board!");
                        BoardState::SleepValidate(board)
                    }
                    None => BoardState::SleepSearch,
                };
            }
            BoardState::Validate(board) => {
                let image = capture_first_screen();

                state = match scanner.scan_image(&image) {
                    Some(confirmation_board) if board == confirmation_board => {
                        println!(" Ready!");
                        BoardState::Ready(board)
                    }
                    _ => {
                        println!(" Board changed!");
                        BoardState::SleepValidate(board)
                    }
                }
            }
            BoardState::Ready(board) => {
                print!("Solving board...");
                stdout().flush().unwrap();
                state = match board.solve(Duration::from_secs(5)) {
                    SolveResult::Solution(solution) => {
                        println!(" Done!");
                        BoardState::Solve(solution)
                    }
                    SolveResult::Timeout => {
                        println!(" Timeout!");
                        println!("Skipping to next game.");
                        mouse::click_next_game(&mouse);
                        BoardState::SleepSearch
                    }
                    SolveResult::Unsolvable => {
                        println!(" Unsolvable!");
                        BoardState::SleepUnsolvable(board)
                    }
                };
            }
            BoardState::Solve(steps) => {
                print!("Applying solution in-game...");
                stdout().flush().unwrap();

                for Step([step1, step2]) in steps {
                    mouse::click_at_coord(&mouse, step1);
                    if step2 != step1 {
                        mouse::click_at_coord(&mouse, step2);
                    }
                }

                mouse::click_next_game(&mouse);

                println!(" Done!");

                state = BoardState::SleepSearch;
            }
            BoardState::Unsolvable(unsolvable_board) => {
                let image = capture_first_screen();

                state = match scanner.scan_image(&image) {
                    Some(board) if board == unsolvable_board => BoardState::SleepUnsolvable(board),
                    Some(board) => {
                        println!("Found a valid board!");
                        BoardState::SleepValidate(board)
                    }
                    _ => BoardState::SleepSearch,
                };
            }
        }
    }
}

fn capture_first_screen() -> RgbImage {
    // image::open(".example.png").unwrap().into_rgb8()
    let screens = Screen::all().unwrap();
    let screen = screens.first().expect("no screen");
    DynamicImage::from(screen.capture().unwrap()).to_rgb8()
}
