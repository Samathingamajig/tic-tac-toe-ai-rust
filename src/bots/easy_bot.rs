use crate::bots::tic_tac_toe_bot::TicTacToeBot;
use crate::types::{Board, Position, Tile};
use crate::util::{get_valid_moves, determine_winner};
use rand::prelude::SliceRandom;

pub struct EasyBot;

impl TicTacToeBot for EasyBot {
    fn next_move(&self, board: &Board, turn: &Tile) -> Position {
        let mut rng = rand::thread_rng();
        let valid_moves = get_valid_moves(&board);

        let mut instant_wins = valid_moves.iter().filter(|(i, j)| {
            let mut b = board.clone();
            b[*i][*j] = *turn;
            match determine_winner(&b) {
                Some(t) => t == *turn,
                _ => false,
            }
        });

        if let Some(p) = instant_wins.next() {
            return *p;
        }

        let other_turn = if *turn == Tile::X { Tile::O } else { Tile::X };
        let mut instant_loss = valid_moves.iter().filter(|(i, j)| {
            let mut b = board.clone();
            b[*i][*j] = other_turn;
            match determine_winner(&b) {
                Some(t) => t == other_turn,
                _ => false,
            }
        });
        if let Some(p) = instant_loss.next() {
            return *p;
        };

        *valid_moves.choose(&mut rng).expect("No valid moves")
    }
}
