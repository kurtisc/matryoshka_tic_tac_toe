extern crate rayon;
use crate::game::{Axis, Game, PlayerKind, Winner};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};

enum Symmetry {
    NoSymmetry,
    Rotate90,
    Rotate180,
    Rotate270,
    FlipH,
    FlipV,
}

#[derive(Clone)]
pub struct Solver {
    move_lookup: Lookup,
    write_lookup: Arc<Mutex<Lookup>>,
    pub kind: PlayerKind,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            move_lookup: Lookup::new(),
            write_lookup: Arc::new(Mutex::new(Lookup::new())),
            kind: PlayerKind::O,
        }
    }

    pub fn find_move(self: &Self, game: &Game) -> (usize, usize, usize) {
        let before_state = game.clone();

        match self.check_lookup(&before_state) {
            Some(s) => self.get_lookup(game, s),
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

                self.add_to_lookup(&game, (i, j, k));
                (i, j, k)
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
                    let score = self.max_search(&x, alpha, beta);

                    beta = min(score, beta);
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
        if game.piece_can_be_placed_at(&piece, i, j) {
            let new_game = game.clone().make_move(i, j, piece);

            match new_game {
                Ok(x) => {
                    let score = self.min_search(&x, alpha, beta);

                    alpha = max(score, alpha);
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

    fn check_lookup(self: &Self, game: &Game) -> Option<Symmetry> {
        let flip_h_game = game.flip_horizontally();
        if self.move_lookup.x.contains_key(&flip_h_game) {
            return Some(Symmetry::FlipH);
        }

        let flip_v_game = game.flip_vertically();
        if self.move_lookup.x.contains_key(&flip_v_game) {
            return Some(Symmetry::FlipV);
        }

        if self.move_lookup.x.contains_key(&game) {
            return Some(Symmetry::NoSymmetry);
        }

        let rotated_game = game.rotate_by_90();
        if self.move_lookup.x.contains_key(&rotated_game) {
            return Some(Symmetry::Rotate90);
        }

        let rotated_game = game.rotate_by_180();
        if self.move_lookup.x.contains_key(&rotated_game) {
            return Some(Symmetry::Rotate180);
        }

        let rotated_game = game.rotate_by_270();
        if self.move_lookup.x.contains_key(&rotated_game) {
            return Some(Symmetry::Rotate270);
        }

        None
    }

    fn get_lookup(self: &Self, game: &Game, symmetry: Symmetry) -> (usize, usize, usize) {
        match symmetry {
            Symmetry::NoSymmetry => self.move_lookup.x[&game],
            Symmetry::Rotate270 => self.move_lookup.x[&game.clone().rotate_by_90()],
            Symmetry::Rotate180 => self.move_lookup.x[&game.clone().rotate_by_180()],
            Symmetry::Rotate90 => self.move_lookup.x[&game.clone().rotate_by_270()],
            Symmetry::FlipH => self.move_lookup.x[&game.clone().flip_horizontally()],
            Symmetry::FlipV => self.move_lookup.x[&game.clone().flip_vertically()],
        }
    }

    fn add_to_lookup(self: &Self, game: &Game, ideal_move: (usize, usize, usize)) {
        let mut shared_lookup = self.write_lookup.lock().unwrap();
        shared_lookup.x.insert(game.clone(), ideal_move);
        shared_lookup.write()
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Lookup {
    #[serde_as(as = "Vec<(_, _)>")]
    x: HashMap<Game, (usize, usize, usize)>,
}

impl Lookup {
    fn write(self: &Self) {
        let mut f = File::create("data/lookup.json").unwrap();
        f.write_all(&serde_json::to_string(self).unwrap().into_bytes())
            .unwrap();
        f.sync_all().unwrap();
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
