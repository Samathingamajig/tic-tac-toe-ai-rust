use crate::bots::tic_tac_toe_bot::TicTacToeBot;
use crate::types::{Board, Position, Tile};
use crate::util::{determine_winner, get_valid_moves};
use rand::prelude::SliceRandom;

pub struct MasterBot;

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
        alpha: i32,
        beta: i32,
    ) -> i32 {
        if let Some(score) = self.score(board, turn, depth) {
            return score;
        }

        let mut alpha = alpha;
        let mut beta = beta;

        let valid_moves = get_valid_moves(board);

        if valid_moves.len() == 0 {
            return self.score(board, turn, depth).unwrap_or(0);
        }

        if is_maximizing_player {
            let mut value = -100_000;
            for (i, j) in get_valid_moves(board) {
                board[i][j] = *turn;
                let score = self.minimax(board, turn, false, depth + 1, alpha, beta);
                board[i][j] = Tile::Empty;
                if score > value {
                    value = score
                }
                if value > alpha {
                    alpha = value;
                }
                if value >= beta {
                    break;
                }
            }
            value
        } else {
            let mut value = 100_000;
            for (i, j) in get_valid_moves(board) {
                board[i][j] = if *turn == Tile::X { Tile::O } else { Tile::X };
                let score = self.minimax(board, turn, true, depth + 1, alpha, beta);
                board[i][j] = Tile::Empty;
                if score < value {
                    value = score
                }
                if value < beta {
                    beta = value;
                }
                if value <= alpha {
                    break;
                }
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
                let score = self.minimax(&mut b, turn, false, 0, std::i32::MIN, std::i32::MAX);
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
