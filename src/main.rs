use std::fmt;
use std::io;
// use rand::Rng;

const WIDTH: usize = 3;
const HEIGHT: usize = 3;

type Board = [[Tile; WIDTH]; HEIGHT];

#[derive(Clone, Copy, PartialEq)]
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
    let mut board: Board = [[Tile::Empty; WIDTH]; HEIGHT];
    let mut turn: Tile = Tile::X;
    loop {
        print_board(&board);
        let selection = match get_selection(&board) {
            Some(n) => n,
            None => break,
        };
        println!("Have selection {:?}", selection);
        board[selection.0][selection.1] = turn;
        turn = if turn == Tile::X { Tile::O } else { Tile::X };
    }
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
        if !(1..=(HEIGHT * WIDTH)).contains(&selection) {
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
            for _ in 0..(WIDTH - 1) {
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
    7 - ind.0 * WIDTH + ind.1
}
