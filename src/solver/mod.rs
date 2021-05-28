extern crate rayon;
use crate::game::{Axis, Game, Winner};
use rayon::prelude::*;
use std::cmp::Ordering;

#[derive(Eq, Copy, Clone)]
struct BestMove {
    b_move: (usize, usize, usize),
    score: i64,
}

impl Ord for BestMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for BestMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BestMove {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

pub fn get_min_max_move(game: &Game) -> (usize, usize, usize) {
    game.symmetry_range(Axis::Horizontal)
        .par_iter()
        .map(|x| min_max_outer_loop(game, *x))
        .collect::<Vec<BestMove>>()
        .par_iter()
        .max()
        .unwrap()
        .b_move
}

fn min_max_outer_loop(game: &Game, i: usize) -> BestMove {
    *game
        .symmetry_range(Axis::Vertical)
        .par_iter()
        .map(|x| min_max_inner_loop(game, i, *x))
        .collect::<Vec<BestMove>>()
        .par_iter()
        .max()
        .unwrap()
}

fn min_max_inner_loop(game: &Game, i: usize, j: usize) -> BestMove {
    let (_, o) = game.players.clone();

    *o.pieces
        .par_iter()
        .map(|x| min_max_loop(game, i, j, *x))
        .collect::<Vec<BestMove>>()
        .par_iter()
        .max()
        .unwrap()
}

fn min_max_loop(game: &Game, i: usize, j: usize, piece: usize) -> BestMove {
    let mut best_score = std::i64::MIN;
    let mut best_move = (0, 0, 0);

    if game.piece_can_be_placed_at(&piece, i, j) {
        let new_game = game.clone().make_move(i, j, piece);

        match new_game {
            Ok(x) => {
                let tmp_score = min_search(&x, std::i64::MIN, std::i64::MAX);

                if tmp_score > best_score {
                    best_score = tmp_score;
                    best_move = (i, j, piece);
                }
            }
            _ => (),
        }
    }

    BestMove {
        b_move: best_move,
        score: best_score,
    }
}

fn min_loop(game: &Game, alpha: i64, mut beta: i64, i: usize, j: usize, piece: usize) -> i64 {
    if game.piece_can_be_placed_at(&piece, i, j) {
        let new_game = game.clone().make_move(i, j, piece);

        match new_game {
            Ok(x) => {
                let score = max_search(&x, alpha, beta);

                if score < beta {
                    beta = score;
                }
                if alpha > beta {
                    return beta;
                }
            }
            _ => (),
        }
    }
    beta
}

fn min_inner_loop(game: &Game, alpha: i64, beta: i64, i: usize, j: usize) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    let (x, _) = &game.players;

    *x.pieces
        .par_iter()
        .map(|x| min_loop(&game, alpha, beta, i, j, *x))
        .collect::<Vec<i64>>()
        .par_iter()
        .min()
        .unwrap()
}

fn min_outer_loop(game: &Game, alpha: i64, beta: i64, i: usize) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    *game
        .symmetry_range(Axis::Vertical)
        .par_iter()
        .map(|x| min_inner_loop(&game, alpha, beta, i, *x))
        .collect::<Vec<i64>>()
        .par_iter()
        .min()
        .unwrap()
}

fn min_search(game: &Game, alpha: i64, beta: i64) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    *game
        .symmetry_range(Axis::Horizontal)
        .par_iter()
        .map(|x| min_outer_loop(&game, alpha, beta, *x))
        .collect::<Vec<i64>>()
        .par_iter()
        .min()
        .unwrap()
}

fn max_loop(game: &Game, mut alpha: i64, beta: i64, i: usize, j: usize, piece: usize) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    if game.piece_can_be_placed_at(&piece, i, j) {
        let new_game = game.clone().make_move(i, j, piece);

        match new_game {
            Ok(x) => {
                let score = min_search(&x, alpha, beta);

                if score > alpha {
                    alpha = score;
                }
                if alpha > beta {
                    return alpha;
                }
            }
            _ => (),
        }
    }
    alpha
}

fn max_inner_loop(game: &Game, alpha: i64, beta: i64, i: usize, j: usize) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }
    let (_, o) = &game.players;

    *o.pieces
        .par_iter()
        .map(|x| max_loop(game, alpha, beta, i, j, *x))
        .collect::<Vec<i64>>()
        .par_iter()
        .max()
        .unwrap()
}

fn max_outer_loop(game: &Game, alpha: i64, beta: i64, i: usize) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    *game
        .symmetry_range(Axis::Vertical)
        .par_iter()
        .map(|x| max_inner_loop(&game, alpha, beta, i, *x))
        .collect::<Vec<i64>>()
        .par_iter()
        .max()
        .unwrap()
}

fn max_search(game: &Game, alpha: i64, beta: i64) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    *game
        .symmetry_range(Axis::Horizontal)
        .par_iter()
        .map(|x| max_outer_loop(&game, alpha, beta, *x))
        .collect::<Vec<i64>>()
        .par_iter()
        .max()
        .unwrap()
}
