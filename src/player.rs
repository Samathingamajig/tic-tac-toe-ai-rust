use crate::types::{Board, Position, Tile};

pub enum Players {
    Human,
    RandomBot,
    EasyBot,
    MasterBot,
}

pub trait Player {
    fn next_move(&self, board: &Board, turn: &Tile) -> Position;
}
