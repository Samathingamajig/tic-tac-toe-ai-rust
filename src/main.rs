use std::fmt;
use std::io;
// use rand::Rng;

const SIZE: usize = 3;

type Board = [[Tile; SIZE]; SIZE];

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tile {
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

fn main() {
    let mut board: Board = [[Tile::Empty; SIZE]; SIZE];
    let mut turn_number = 0;
    let winner: Option<Tile> = loop {
        print_board(&board);
        let selection = match get_selection(&board) {
            Some(n) => n,
            None => break None,
        };
        println!("Have selection {:?}", selection);
        board[selection.0][selection.1] = if turn_number % 2 == 0 {
            Tile::X
        } else {
            Tile::O
        };
        let winner = determine_winner(board);
        if winner.is_some() {
            break winner;
        };
        turn_number += 1;
        if turn_number == SIZE * SIZE {
            break None;
        }
    };
    print_board(&board);
    match winner {
        None => println!("Welp, it's a tie"),
        Some(player) => println!("Congratulations player {}, you won!", player),
    };
}

fn get_selection(board: &Board) -> Option<(usize, usize)> {
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Error reading from stdin");
        let selection: usize = match buffer.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                if buffer.trim() == "exit" {
                    return None;
                }
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
        return Some(selection);
    }
}

fn print_board(board: &Board) {
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

fn position_to_indicies(pos: usize) -> (usize, usize) {
    (2 - (pos - 1) / 3, (pos - 1) % 3)
}

fn indicies_to_position(ind: (usize, usize)) -> usize {
    7 - ind.0 * SIZE + ind.1
}

fn determine_winner(board: Board) -> Option<Tile> {
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
