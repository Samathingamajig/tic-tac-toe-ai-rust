use super::tic_tac_toe_bot::TicTacToeBot;
use crate::tic_tac_toe_player::TicTacToePlayer;
use crate::types::{Board, Position, Tile};
use crate::util::get_valid_moves;
use rand::prelude::SliceRandom;

pub struct RandomBot;

impl TicTacToeBot for RandomBot {}

impl TicTacToePlayer for RandomBot {
    fn next_move(&self, board: &Board, _turn: &Tile) -> Position {
        let mut rng = rand::thread_rng();
        *get_valid_moves(&board)
            .choose(&mut rng)
            .expect("No valid moves")
    }
}
