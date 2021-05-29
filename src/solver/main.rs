// Above 4, the solver will take a very long time to run
const SOLVER_NUMBER_OF_PIECES: usize = 5;

use matryoshka_tic_tac_toe::game::{Game, PlayerKind, Winner};
use matryoshka_tic_tac_toe::io::{print_tiles, prompt_move};
use matryoshka_tic_tac_toe::solver::Solver;

fn main() {
    let mut game = Game::new_with_size(SOLVER_NUMBER_OF_PIECES);
    let mut solver = Solver::new();

    while !game.is_finished() {
        print_tiles(game.tiles());
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
            PlayerKind::O => {
                let (a, b) = solver.find_move(&game);
                solver = a;
                let (i, j, k) = b;
                (i, j, k)
            }
        };

        match game.clone().make_move(row, col, size) {
            Ok(x) => game = x,
            _ => continue,
        }
    }

    print_tiles(game.tiles());

    match game.winner().expect("A finished game should have winner") {
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }
}
