// For 7 pieces, the solver will take a multiple days to run
const SOLVER_NUMBER_OF_PIECES: usize = 6;

extern crate getopts;
use getopts::Options;
use matryoshka_tic_tac_toe::game::{Game, PlayerKind, Winner};
use matryoshka_tic_tac_toe::io::*;
use matryoshka_tic_tac_toe::solver::Solver;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

fn main() {
    let solver = Solver::new();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut number = SOLVER_NUMBER_OF_PIECES;

    let mut opts = Options::new();
    opts.optopt(
        "n",
        "number",
        "How many pieces to use",
        "PIECES",
    );

    opts.optflag("h", "help", "print this help menu");
    match opts.parse(&args[1..]) {
        Ok(m) => {
            if m.opt_present("h") {
                print_usage(&program, opts);
                return;
            }
            if m.opt_present("n") {
                match m.opt_get::<usize>("n") {
                    Ok(Some(n)) => number = n,
                    _ => (),
                }
            }
        }
        _ => (),
    };
    let mut game = Game::new_with_size(number);

    while !game.is_finished() {
        game.tiles().print();
        let (x, o) = game.players.clone();
        x.print_pieces();
        o.print_pieces();

        println!(
            "Next piece: {}",
            match game.current_player_kind() {
                PlayerKind::X => "x",
                PlayerKind::O => "o",
            }
        );

        let (row, col, size) = match game.current_player_kind() {
            PlayerKind::X => prompt_move(),
            PlayerKind::O => solver.find_move(&game),
        };

        match game.clone().make_move(row, col, size) {
            Ok(x) => game = x,
            _ => continue,
        }
    }

    game.tiles().print();

    match game.winner().expect("A finished game should have winner") {
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }
}
