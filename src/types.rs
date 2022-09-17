pub const SIZE: usize = 3;
pub type Position = (usize, usize);
use std::fmt;

pub type Board = [[Tile; SIZE]; SIZE];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tile {
    Empty,
    X,
    O,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match *self {
            Tile::Empty => " ",
            Tile::X => "X",
            Tile::O => "O",
        })
        .expect("Failed printing to formatter");
        Ok(())
    }
}
