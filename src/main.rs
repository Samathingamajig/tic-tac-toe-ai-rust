mod bots;
mod types;
mod util;
mod tic_tac_toe_player;
mod selection;
mod human;
use crate::types::{Board, Tile, SIZE};
use crate::util::{determine_winner, print_board};
use crate::selection::choose_player;

fn main() {
    let mut board: Board = [[Tile::Empty; SIZE]; SIZE];
    let mut turn_number = 0;
    let player1 = choose_player(1).expect("No player1 selected");
    println!();
    let player2 = choose_player(2).expect("No player2 selected");
    println!();
    let winner: Option<Tile> = loop {
        print_board(&board);
        let turn = if turn_number % 2 == 0 {
            Tile::X
        } else {
            Tile::O
        };
        let selection = if turn_number % 2 == 0 {
            player1.next_move(&board, &turn)
        } else {
            player2.next_move(&board, &turn)
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
