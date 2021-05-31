extern crate getopts;
use getopts::Options;
use matryoshka_tic_tac_toe::game::Game;
use matryoshka_tic_tac_toe::solver::Solver;
use std::env;
use std::time::SystemTime;

#[derive(Debug, Clone)]
struct Progress {
    number_of_pieces: usize,
    row: usize,
    col: usize,
    piece: usize,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let solver = Solver::new_overwrite_lookup();
    let mut progress = Progress {
        number_of_pieces: 0,
        row: 0,
        col: 0,
        piece: 0,
    };
    let mut until: usize = 7;
    let mut deep = false;

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "u",
        "until",
        "Run the generator until this many pieces have been solved",
        "PIECES",
    );

    opts.optflag("d", "deep", "Run the generator past the top-level moves");

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
            if m.opt_present("d") {
                deep = true;
            }
        }
        _ => (),
    };

    for number_of_pieces in 3..until + 1 {
        for row in 0..3 {
            for col in 0..3 {
                for piece in 0..number_of_pieces {
                    progress.number_of_pieces = number_of_pieces;
                    progress.row = row;
                    progress.col = col;
                    progress.piece = piece;
                    println! {"{:?}", progress};

                    let now = SystemTime::now();
                    let mut game = Game::new_with_size(number_of_pieces);
                    game = game.make_move(row, col, piece).unwrap();
                    let (i, j, k) = solver.find_move(&game);
                    let mut game = game.make_move(i, j, k).unwrap();
                    if deep {
                        do_next_lookup(&mut game, &solver, &number_of_pieces);
                    }

                    println! {"{}", now.elapsed().unwrap().as_secs()};
                }
            }
        }
    }
}

fn do_next_lookup(game: &mut Game, solver: &Solver, number_of_pieces: &usize) {
    for row in 0..3 {
        for col in 0..3 {
            for piece in 0..*number_of_pieces {
                if game.get_turn_count() > 4 {
                    return;
                }
                let new_game = game.clone().make_move(row, col, piece);
                match new_game {
                    Ok(_) => {
                        let mut game = new_game.unwrap();
                        if game.is_finished() {
                            return;
                        }
                        let (i, j, k) = solver.find_move(&game);
                        let new_game = game.clone().make_move(i, j, k);
                        match new_game {
                            Ok(_) => {
                                game = game.make_move(i, j, k).unwrap();
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
