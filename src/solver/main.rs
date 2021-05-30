// For 6 pieces, the solver will take a multiple days to run
const SOLVER_NUMBER_OF_PIECES: usize = 5;

use matryoshka_tic_tac_toe::game::{Game, PlayerKind, Winner};
use matryoshka_tic_tac_toe::io::*;
use matryoshka_tic_tac_toe::solver::Solver;

fn main() {
    let mut game = Game::new_with_size(SOLVER_NUMBER_OF_PIECES);
    let solver = Solver::new();

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
