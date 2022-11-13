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
    println!("Tic-Tac-Toe AI");
    println!("by Samuel Gunter");
    println!("--------------------");
    println!();
    let mut board: Board = [[Tile::Empty; SIZE]; SIZE];
    let mut turn_number = 0;
    let player_x = choose_player(Tile::X);
    println!();
    let player_o = choose_player(Tile::O);
    println!();
    let winner: Option<Tile> = loop {
        print_board(&board);
        let turn = if turn_number % 2 == 0 {
            Tile::X
        } else {
            Tile::O
        };
        let selection = if turn_number % 2 == 0 {
            player_x.next_move(&board, &turn)
        } else {
            player_o.next_move(&board, &turn)
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
