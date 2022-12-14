use crate::types::{Board, Position, Tile, SIZE};

pub fn print_board(board: &Board) {
    for (i, row) in board.iter().enumerate() {
        if i != 0 {
            for _ in 0..(SIZE - 1) {
                print!("-+");
            }
            println!("-");
        }
        for (j, tile) in row.iter().enumerate() {
            if j != 0 {
                print!("|");
            };

            if *tile == Tile::Empty {
                print!("{}", indicies_to_position((i, j)));
            } else {
                print!("{}", tile);
            };
        }
        println!();
    }
}

pub fn position_to_indicies(pos: usize) -> Position {
    (SIZE - 1 - (pos - 1) / SIZE, (pos - 1) % SIZE)
}

pub fn indicies_to_position(ind: Position) -> usize {
    7 - ind.0 * SIZE + ind.1
}

pub fn determine_winner(board: &Board) -> Option<Tile> {
    for player in [Tile::X, Tile::O] {
        // Check rows
        for row in board {
            if row.iter().all(|t| *t == player) {
                return Some(player);
            };
        }

        // Check cols
        for col_idx in 0..SIZE {
            if board.map(|row| row[col_idx]).iter().all(|t| *t == player) {
                return Some(player);
            };
        }

        // Check diagonals
        if board
            .iter()
            .enumerate()
            .map(|(i, row)| row[i])
            .all(|t| t == player)
        {
            return Some(player);
        };
        if board
            .iter()
            .enumerate()
            .map(|(i, row)| row[SIZE - i - 1])
            .all(|t| t == player)
        {
            return Some(player);
        };
    }
    None
}

pub fn get_valid_moves(board: &Board) -> Vec<(usize, usize)> {
    board
        .iter()
        .enumerate()
        .map(move |(i, row)| row.iter().enumerate().map(move |(j, v)| (i, j, v)))
        .flatten()
        .filter(|(_i, _j, v)| **v == Tile::Empty)
        .map(|(i, j, _v)| (i, j))
        .collect()
}
