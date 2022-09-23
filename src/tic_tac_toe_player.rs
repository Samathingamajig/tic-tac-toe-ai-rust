use crate::types::{Board, Position, Tile};
use strum::{EnumIter, EnumString};

#[derive(Debug, EnumIter, EnumString)]
pub enum Players {
    Human,
    RandomBot,
    EasyBot,
    MasterBot,
}

pub trait TicTacToePlayer {
    fn next_move(&self, board: &Board, turn: &Tile) -> Position;
}
