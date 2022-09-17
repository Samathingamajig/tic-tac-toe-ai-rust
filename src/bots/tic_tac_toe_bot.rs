use crate::types::{Board, Position, Tile};

pub trait TicTacToeBot {
    fn next_move(&self, board: &Board, turn: &Tile) -> Position;
}
