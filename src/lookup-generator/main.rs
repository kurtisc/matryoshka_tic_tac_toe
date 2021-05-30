extern crate getopts;
use getopts::Options;
use matryoshka_tic_tac_toe::game::Game;
use matryoshka_tic_tac_toe::solver::Solver;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let solver = Solver::new_overwrite_lookup();
    let mut progress = Progress::new();
    let mut resumed = false;
    let mut until: usize = 7;

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "u",
        "until",
        "Run the generator until this many pieces have been solved",
        "PIECES",
    );
    opts.optflag("h", "help", "print this help menu");
    match opts.parse(&args[1..]) {
        Ok(m) => {
            if m.opt_present("h") {
                print_usage(&program, opts);
                return;
            }
            if m.opt_present("u") {
                match m.opt_get::<usize>("u") {
                    Ok(Some(u)) => until = u,
                    _ => (),
                }
            }
        }
        _ => (),
    };

    for mut number_of_pieces in 3..until + 1 {
        for mut row in 0..3 {
            for mut col in 0..3 {
                for mut piece in 0..number_of_pieces {
                    if !resumed {
                        number_of_pieces = progress.number_of_pieces;
                        row = progress.row;
                        col = progress.col;
                        piece = progress.piece;
                        resumed = true;
                    }

                    progress.number_of_pieces = number_of_pieces;
                    progress.row = row;
                    progress.col = col;
                    progress.piece = piece;
                    progress.write();
                    println! {"{:?}", progress};

                    let now = SystemTime::now();
                    let mut game = Game::new_with_size(number_of_pieces);
                    game = game.make_move(row, col, piece).unwrap();
                    let (i, j, k) = solver.find_move(&game);
                    let game = game.make_move(i, j, k).unwrap();
                    do_next_lookup(&game, &solver, &number_of_pieces);

                    println! {"{}", now.elapsed().unwrap().as_secs()};
                }
            }
        }
    }
}

fn do_next_lookup(game: &Game, solver: &Solver, number_of_pieces: &usize) {
    let mut game = game.clone();
    while !game.is_finished() {
        for row in 0..3 {
            for col in 0..3 {
                for piece in 0..*number_of_pieces {
                    let new_game = game.clone().make_move(row, col, piece);
                    match new_game {
                        Ok(_) => {
                            game = game.make_move(row, col, piece).unwrap();
                            if game.is_finished() {
                                return;
                            }
                            let (i, j, k) = solver.find_move(&game);
                            let new_game = game.clone().make_move(i, j, k);
                            match new_game {
                                Ok(_) => {
                                    game = game.clone().make_move(i, j, k).unwrap();
                                    if game.is_finished() {
                                        return;
                                    }
                                }
                                _ => (),
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Progress {
    number_of_pieces: usize,
    row: usize,
    col: usize,
    piece: usize,
}

impl Progress {
    fn write(self: &Self) {
        let mut f = File::create("data/generator-progress.json").unwrap();
        f.write_all(&serde_json::to_string(self).unwrap().into_bytes())
            .unwrap();
        f.sync_all().unwrap();
    }

    fn new() -> Self {
        let f = File::open("data/generator-progress.json");
        match f {
            Ok(mut file) => {
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => serde_json::from_str(&contents.to_string()).unwrap(),
                    _ => Progress {
                        number_of_pieces: 3,
                        row: 0,
                        col: 0,
                        piece: 0,
                    },
                }
            }
            _ => Progress {
                number_of_pieces: 3,
                row: 0,
                col: 0,
                piece: 0,
            },
        }
    }
}
