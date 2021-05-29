extern crate rayon;
use crate::game::{Axis, Game, PlayerKind, Winner};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone)]
pub struct Solver {
    move_lookup: Lookup,
    pub kind: PlayerKind,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            move_lookup: Lookup::new(),
            kind: PlayerKind::O,
        }
    }

    pub fn find_move(mut self: Self, game: &Game) -> (Self, (usize, usize, usize)) {
        let before_state = game.clone();

        match self.move_lookup.x.contains_key(&before_state) {
            true => (self.clone(), self.move_lookup.x[&before_state]),
            _ => {
                let (i, j, k) = game
                    .symmetry_range(Axis::Horizontal)
                    .par_iter()
                    .map(|x| self.min_max_outer_loop(game, *x))
                    .collect::<Vec<BestMove>>()
                    .par_iter()
                    .max()
                    .unwrap()
                    .b_move;

                self = self.add_to_lookup(&before_state, (i, j, k));
                (self, (i, j, k))
            }
        }
    }

    fn min_max_outer_loop(self: &Self, game: &Game, i: usize) -> BestMove {
        *game
            .symmetry_range(Axis::Vertical)
            .par_iter()
            .map(|x| self.min_max_inner_loop(game, i, *x))
            .collect::<Vec<BestMove>>()
            .par_iter()
            .max()
            .unwrap()
    }

    fn min_max_inner_loop(self: &Self, game: &Game, i: usize, j: usize) -> BestMove {
        let my = match self.kind {
            PlayerKind::O => {
                let (_, o) = game.players.clone();
                o
            }
            PlayerKind::X => {
                let (x, _) = game.players.clone();
                x
            }
        };

        *my.pieces
            .par_iter()
            .map(|y| self.min_max_loop(game, i, j, *y))
            .collect::<Vec<BestMove>>()
            .par_iter()
            .max()
            .unwrap()
    }

    fn min_max_loop(self: &Self, game: &Game, i: usize, j: usize, piece: usize) -> BestMove {
        let mut best_score = std::i64::MIN;
        let mut best_move = (0, 0, 0);

        if game.piece_can_be_placed_at(&piece, i, j) {
            let new_game = game.clone().make_move(i, j, piece);

            match new_game {
                Ok(x) => {
                    let tmp_score = self.min_search(&x, std::i64::MIN, std::i64::MAX);

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

    fn min_loop(
        self: &Self,
        game: &Game,
        alpha: i64,
        mut beta: i64,
        i: usize,
        j: usize,
        piece: usize,
    ) -> i64 {
        if game.piece_can_be_placed_at(&piece, i, j) {
            let new_game = game.clone().make_move(i, j, piece);

            match new_game {
                Ok(x) => {
                    let score = self.max_search(&x, alpha, beta);

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

    fn min_inner_loop(self: &Self, game: &Game, alpha: i64, beta: i64, i: usize, j: usize) -> i64 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        let their = match self.kind {
            PlayerKind::O => {
                let (x, _) = game.players.clone();
                x
            }
            PlayerKind::X => {
                let (_, o) = game.players.clone();
                o
            }
        };

        *their
            .pieces
            .par_iter()
            .map(|x| self.min_loop(&game, alpha, beta, i, j, *x))
            .collect::<Vec<i64>>()
            .par_iter()
            .min()
            .unwrap()
    }

    fn min_outer_loop(self: &Self, game: &Game, alpha: i64, beta: i64, i: usize) -> i64 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        *game
            .symmetry_range(Axis::Vertical)
            .par_iter()
            .map(|x| self.min_inner_loop(&game, alpha, beta, i, *x))
            .collect::<Vec<i64>>()
            .par_iter()
            .min()
            .unwrap()
    }

    fn min_search(self: &Self, game: &Game, alpha: i64, beta: i64) -> i64 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        *game
            .symmetry_range(Axis::Horizontal)
            .par_iter()
            .map(|x| self.min_outer_loop(&game, alpha, beta, *x))
            .collect::<Vec<i64>>()
            .par_iter()
            .min()
            .unwrap()
    }

    fn max_loop(
        self: &Self,
        game: &Game,
        mut alpha: i64,
        beta: i64,
        i: usize,
        j: usize,
        piece: usize,
    ) -> i64 {
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
                    let score = self.min_search(&x, alpha, beta);

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

    fn max_inner_loop(self: &Self, game: &Game, alpha: i64, beta: i64, i: usize, j: usize) -> i64 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        let my = match self.kind {
            PlayerKind::O => {
                let (_, o) = game.players.clone();
                o
            }
            PlayerKind::X => {
                let (x, _) = game.players.clone();
                x
            }
        };

        *my.pieces
            .par_iter()
            .map(|x| self.max_loop(game, alpha, beta, i, j, *x))
            .collect::<Vec<i64>>()
            .par_iter()
            .max()
            .unwrap()
    }

    fn max_outer_loop(self: &Self, game: &Game, alpha: i64, beta: i64, i: usize) -> i64 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        *game
            .symmetry_range(Axis::Vertical)
            .par_iter()
            .map(|x| self.max_inner_loop(&game, alpha, beta, i, *x))
            .collect::<Vec<i64>>()
            .par_iter()
            .max()
            .unwrap()
    }

    fn max_search(self: &Self, game: &Game, alpha: i64, beta: i64) -> i64 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        *game
            .symmetry_range(Axis::Horizontal)
            .par_iter()
            .map(|x| self.max_outer_loop(&game, alpha, beta, *x))
            .collect::<Vec<i64>>()
            .par_iter()
            .max()
            .unwrap()
    }

    fn add_to_lookup(mut self: Self, game: &Game, ideal_move: (usize, usize, usize)) -> Self {
        self.move_lookup.x.insert(game.clone(), ideal_move);
        self.clone().write_lookup();
        self
    }

    fn write_lookup(self: Self) -> () {
        self.move_lookup.write_lookup()
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Lookup {
    #[serde_as(as = "Vec<(_, _)>")]
    x: HashMap<Game, (usize, usize, usize)>,
}

impl Lookup {
    fn write_lookup(self: &Self) {
        let mut f = File::create("data/new_lookup.json").unwrap();
        f.write_all(&serde_json::to_string(self).unwrap().into_bytes())
            .unwrap();
    }

    fn new() -> Self {
        let f = File::open("data/lookup.json");
        match f {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => serde_json::from_str(&contents.to_string()).unwrap(),
                    _ => Lookup { x: HashMap::new() },
                }
            }
            _ => Lookup { x: HashMap::new() },
        }
    }
}

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
