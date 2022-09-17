mod bots;
mod types;
mod util;
use crate::bots::master_bot::MasterBot;
use crate::bots::tic_tac_toe_bot::TicTacToeBot;
use crate::types::{Board, Tile, SIZE};
use crate::util::{determine_winner, get_selection, indicies_to_position, print_board};

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
        let selection = if turn_number % 2 == 0 {
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
