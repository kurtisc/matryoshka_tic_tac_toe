extern crate rayon;
use crate::game::{Game, PlayerKind, Winner};
use crate::symmetry::*;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use strum::IntoEnumIterator;

#[derive(Clone)]
pub struct Solver {
    move_lookup: Lookup,
    write_lookup: Option<Arc<Mutex<Lookup>>>,
    pub kind: PlayerKind,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            move_lookup: Lookup::new(),
            write_lookup: None,
            kind: PlayerKind::O,
        }
    }

    pub fn new_overwrite_lookup() -> Self {
        Self {
            move_lookup: Lookup::new(),
            write_lookup: Some(Arc::new(Mutex::new(Lookup::new()))),
            kind: PlayerKind::O,
        }
    }

    pub fn find_move(self: &Self, game: &Game) -> (usize, usize, usize) {
        let before_state = game.clone();

        match self.check_lookup(&before_state) {
            Some(ijk) => ijk,
            _ => {
                let (i, j, k) = game
                    .symmetry_range(Symmetry::FlipH)
                    .par_iter()
                    .map(|x| self.min_max_outer_loop(game, *x))
                    .max()
                    .unwrap()
                    .b_move;

                self.add_to_lookup(&game, (i, j, k));
                (i, j, k)
            }
        }
    }

    fn min_max_outer_loop(self: &Self, game: &Game, i: usize) -> BestMove {
        game.symmetry_range(Symmetry::FlipV)
            .par_iter()
            .map(|x| self.min_max_inner_loop(game, i, *x))
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

        my.pieces
            .par_iter()
            .map(|y| self.min_max_loop(game, i, j, *y))
            .max()
            .unwrap()
    }

    fn min_max_loop(self: &Self, game: &Game, i: usize, j: usize, piece: usize) -> BestMove {
        let mut best_score = std::i8::MIN;
        let mut best_move = (0, 0, 0);

        let new_game = game.clone().make_move(i, j, piece);

        match new_game {
            Ok(x) => {
                let tmp_score = self.min_search(&x, std::i8::MIN, std::i8::MAX);

                if tmp_score > best_score {
                    best_score = tmp_score;
                    best_move = (i, j, piece);
                }
            }
            _ => (),
        }

        BestMove {
            b_move: best_move,
            score: best_score,
        }
    }

    fn min_loop(
        self: &Self,
        game: &Game,
        alpha: i8,
        mut beta: i8,
        i: usize,
        j: usize,
        piece: usize,
    ) -> i8 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

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
        beta
    }

    fn min_inner_loop(self: &Self, game: &Game, alpha: i8, beta: i8, i: usize, j: usize) -> i8 {
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

        their
            .pieces
            .par_iter()
            .map(|x| self.min_loop(&game, alpha, beta, i, j, *x))
            .min()
            .unwrap()
    }

    fn min_outer_loop(self: &Self, game: &Game, alpha: i8, beta: i8, i: usize) -> i8 {
        game.symmetry_range(Symmetry::FlipV)
            .par_iter()
            .map(|x| self.min_inner_loop(&game, alpha, beta, i, *x))
            .min()
            .unwrap()
    }

    fn min_search(self: &Self, game: &Game, alpha: i8, beta: i8) -> i8 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        game.symmetry_range(Symmetry::FlipH)
            .par_iter()
            .map(|x| self.min_outer_loop(&game, alpha, beta, *x))
            .min()
            .unwrap()
    }

    fn max_loop(
        self: &Self,
        game: &Game,
        mut alpha: i8,
        beta: i8,
        i: usize,
        j: usize,
        piece: usize,
    ) -> i8 {
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
        alpha
    }

    fn max_inner_loop(self: &Self, game: &Game, alpha: i8, beta: i8, i: usize, j: usize) -> i8 {
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

        my.pieces
            .par_iter()
            .map(|x| self.max_loop(game, alpha, beta, i, j, *x))
            .max()
            .unwrap()
    }

    fn max_outer_loop(self: &Self, game: &Game, alpha: i8, beta: i8, i: usize) -> i8 {
        game.symmetry_range(Symmetry::FlipV)
            .par_iter()
            .map(|x| self.max_inner_loop(&game, alpha, beta, i, *x))
            .max()
            .unwrap()
    }

    fn max_search(self: &Self, game: &Game, alpha: i8, beta: i8) -> i8 {
        match game.winner() {
            Some(Winner::X) => return -10,
            Some(Winner::O) => return 10,
            Some(Winner::Tie) => return 0,
            _ => (),
        }

        game.symmetry_range(Symmetry::FlipH)
            .par_iter()
            .map(|x| self.max_outer_loop(&game, alpha, beta, *x))
            .max()
            .unwrap()
    }

    fn check_lookup(self: &Self, game: &Game) -> Option<(usize, usize, usize)> {
        if self.move_lookup.x.contains_key(&game) {
            return Some(self.move_lookup.x[&game]);
        }
        for symmetry in Symmetry::iter() {
            let symmetry_game = &game.clone().fliptate(&symmetry);
            match self.move_lookup.x.get(symmetry_game) {
                Some(ijk) => {
                    let (i, j, k) = ijk;
                    let (x, y) = index_to_coordinates(fliptate_coordinates(
                        coordinates_to_index((*i, *j)),
                        &symmetry,
                    ));
                    return Some((x, y, *k));
                }
                _ => (),
            }
        }

        None
    }

    fn add_to_lookup(self: &Self, game: &Game, ideal_move: (usize, usize, usize)) {
        match &self.write_lookup {
            Some(l) => {
                let mut shared_lookup = l.lock().unwrap();
                shared_lookup.x.insert(game.clone(), ideal_move);
                shared_lookup.write()
            }
            _ => (),
        };
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
    score: i8,
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
