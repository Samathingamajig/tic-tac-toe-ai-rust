use crate::types::{Board, Position, Tile};
use strum::{EnumIter, EnumString};
use std::fmt;

#[derive(Debug, EnumIter, EnumString)]
pub enum Players {
    Human,
    RandomBot,
    EasyBot,
    MasterBot,
}

impl fmt::Display for Players {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait TicTacToePlayer {
    fn next_move(&self, board: &Board, turn: &Tile) -> Position;
}
