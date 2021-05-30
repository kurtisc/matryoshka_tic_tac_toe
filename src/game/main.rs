use matryoshka_tic_tac_toe::{game, io};

use game::{Game, PlayerKind, Winner};
use io::*;

fn main() {
    let mut game = Game::new();

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

        let (row, col, size) = prompt_move();

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
