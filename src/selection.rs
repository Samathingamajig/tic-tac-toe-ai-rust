use crate::tic_tac_toe_player::{Players, TicTacToePlayer};
use std::io;
use std::io::prelude::*;
use strum::IntoEnumIterator;
use crate::bots;
use crate::human;

pub fn choose_player(number: i32) -> Option<Box<dyn TicTacToePlayer>> {
    let players: Vec<_> = Players::iter().collect();
    println!("Select player #{}:", number);
    for (i, p) in players.iter().enumerate() {
        println!("  {}. {:?}", i + 1, p);
    }
    let selection = loop {
        let mut buffer = String::new();
        print!("? ");
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
                    break None;
                }
                println!("Error parsing '{}'", buffer.trim());
                continue;
            }
        };
        if !(1..=players.len()).contains(&selection) {
            println!("Not in range");
            continue;
        };
        break Some(selection);
    };

    match selection {
        None => None,
        Some(n) => match &players[n - 1] {
            Players::Human => Some(Box::new(human::Human)),
            Players::RandomBot => Some(Box::new(bots::RandomBot)),
            Players::EasyBot => Some(Box::new(bots::EasyBot)),
            Players::MasterBot => Some(Box::new(bots::MasterBot)),
        },
    }
}
