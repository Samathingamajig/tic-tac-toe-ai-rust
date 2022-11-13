use crate::tic_tac_toe_player::{Players, TicTacToePlayer};
use strum::IntoEnumIterator;
use crate::types::Tile;
use crate::bots;
use crate::human;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};

pub fn choose_player(tile: Tile) -> Box<dyn TicTacToePlayer> {
    let player_options: Vec<_> = Players::iter().collect();

    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Select player for {}", tile))
        .default(0)
        .items(&player_options[..])
        .interact()
        .expect("Failed to select player");

    match &player_options[selection_index] {
        Players::Human => Box::new(human::Human),
        Players::RandomBot => Box::new(bots::RandomBot),
        Players::EasyBot =>Box::new(bots::EasyBot),
        Players::MasterBot => Box::new(bots::MasterBot),
    }
}
