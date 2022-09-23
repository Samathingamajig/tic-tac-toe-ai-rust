use crate::tic_tac_toe_player::TicTacToePlayer;
use crate::types::{Board, Position, Tile, SIZE};
use std::io;
use std::io::prelude::*;
use crate::util::position_to_indicies;

pub struct Human;

impl TicTacToePlayer for Human {
    fn next_move(&self, board: &Board, _turn: &Tile) -> Position {
        loop {
            let mut buffer = String::new();
            print!("Enter your move: ");
            io::stdout()
                .flush()
                .ok()
                .expect("couldn't flush for some reason??");
            io::stdin()
                .read_line(&mut buffer)
                .expect("Error reading from stdin");
            let selection: usize = match buffer.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    // if buffer.trim() == "exit" {
                    //     return None;
                    // }
                    println!("Error parsing '{}'", buffer.trim());
                    continue;
                }
            };
            if !(1..=(SIZE * SIZE)).contains(&selection) {
                println!("Not in range");
                continue;
            };
            let selection = position_to_indicies(selection);
            if board[selection.0][selection.1] != Tile::Empty {
                println!("Not empty");
                continue;
            }
            return selection;
        }
    }
}
