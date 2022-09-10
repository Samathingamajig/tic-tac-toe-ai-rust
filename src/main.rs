use std::fmt;
use std::io;
use std::io::prelude::*;
use rand::seq::SliceRandom;

const SIZE: usize = 3;
type Position = (usize, usize);

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

#[allow(unreachable_code)]
fn main() {
    let mut board: Board = [[Tile::Empty; SIZE]; SIZE];
    let mut turn_number = 0;
    let bot: Box<dyn TicTacToeBot> = Box::new(MasterBot {});
    let winner: Option<Tile> = loop {
        print_board(&board);
        let turn = if turn_number % 2 == 0 {
            Tile::X
        } else {
            Tile::O
        };
        let selection = if turn_number % 2 == 1 {
            let next_move = bot.next_move(&board, &turn);
            println!(
                "Bot: {:?} aka {}",
                next_move,
                indicies_to_position(next_move)
            );
            next_move
        } else {
            match get_selection(&board) {
                Some(n) => n,
                None => break None,
            }
        };
        println!();
        println!("Selected {:?}", selection);
        board[selection.0][selection.1] = turn;
        let winner = determine_winner(&board);
        if winner.is_some() {
            break winner;
        };
        turn_number += 1;
        if turn_number == SIZE * SIZE {
            break None;
        }
    };
    print_board(&board);
    println!();
    match winner {
        None => println!("Welp, it's a tie"),
        Some(player) => println!("Congratulations player {}, you won!", player),
    };
}

fn get_selection(board: &Board) -> Option<Position> {
    loop {
        let mut buffer = String::new();
        print!("Enter your move: ");
        io::stdout()
            .flush()
            .ok()
            .expect("couldn't flush for some reason??");
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

fn position_to_indicies(pos: usize) -> Position {
    (SIZE - 1 - (pos - 1) / SIZE, (pos - 1) % SIZE)
}

fn indicies_to_position(ind: Position) -> usize {
    7 - ind.0 * SIZE + ind.1
}

fn determine_winner(board: &Board) -> Option<Tile> {
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

fn get_valid_moves(board: &Board) -> Vec<(usize, usize)> {
    board
        .iter()
        .enumerate()
        .map(move |(i, row)| row.iter().enumerate().map(move |(j, v)| (i, j, v)))
        .flatten()
        .filter(|(_i, _j, v)| **v == Tile::Empty)
        .map(|(i, j, _v)| (i, j))
        .collect()
}

trait TicTacToeBot {
    fn next_move(&self, board: &Board, turn: &Tile) -> Position;
}

struct RandomBot;

impl TicTacToeBot for RandomBot {
    fn next_move(&self, board: &Board, _turn: &Tile) -> Position {
        let mut rng = rand::thread_rng();
        *get_valid_moves(&board)
            .choose(&mut rng)
            .expect("No valid moves")
    }
}

struct EasyBot;

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

struct MasterBot;

impl MasterBot {
    fn score(&self, board: &Board, turn: &Tile, depth: i32) -> Option<i32> {
        match determine_winner(board) {
            None => None,
            Some(t) => {
                if t == Tile::Empty {
                    Some(0)
                } else if t == *turn {
                    Some(10 - depth)
                } else {
                    Some(depth - 10)
                }
            }
        }
    }

    fn minimax(
        &self,
        board: &mut Board,
        turn: &Tile,
        is_maximizing_player: bool,
        depth: i32,
    ) -> i32 {
        if let Some(score) = self.score(board, turn, depth) {
            return score;
        }

        let valid_moves = get_valid_moves(board);

        if valid_moves.len() == 0 {
            return self.score(board, turn, depth).unwrap_or(0);
        }

        if is_maximizing_player {
            let mut value = -100_000;
            for (i, j) in get_valid_moves(board) {
                board[i][j] = *turn;
                let score = self.minimax(board, turn, false, depth + 1);
                board[i][j] = Tile::Empty;
                if score > value {
                    value = score
                };
            }
            value
        } else {
            let mut value = 100_000;
            for (i, j) in get_valid_moves(board) {
                board[i][j] = if *turn == Tile::X { Tile::O } else { Tile::X };
                let score = self.minimax(board, turn, true, depth + 1);
                board[i][j] = Tile::Empty;
                if score < value {
                    value = score
                };
            }
            value
        }
    }
}

impl TicTacToeBot for MasterBot {
    fn next_move(&self, board: &Board, turn: &Tile) -> Position {
        let mut rng = rand::thread_rng();
        let valid_moves = get_valid_moves(&board);

        let mut move_scores: Vec<(&usize, &usize, i32)> = valid_moves
            .iter()
            .map(|(i, j)| {
                let mut b = board.clone();
                b[*i][*j] = *turn;
                let score = self.minimax(&mut b, turn, false, 0);
                (i, j, score)
            })
            .collect();
        move_scores.sort_by(|a, b| b.2.partial_cmp(&a.2).expect("Can't compare :shrug:"));
        let move_scores = move_scores;

        let best_moves: Vec<&(&usize, &usize, i32)> = move_scores
            .iter()
            .filter(|(_i, _j, score)| *score == move_scores[0].2)
            .collect();
        let best_move = best_moves
            .choose(&mut rng)
            .expect("Expected at least one move");

        (*best_move.0, *best_move.1)
    }
}
