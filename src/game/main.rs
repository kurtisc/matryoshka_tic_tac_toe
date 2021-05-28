use matryoshka_tic_tac_toe::{game, io};

use game::{Game, PlayerKind, Winner};
use io::{print_tiles, prompt_move};

fn main() {
    let mut game = Game::new();

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

        let (row, col, size) = prompt_move();

        game = game.make_move(row, col, size);
    }

    print_tiles(game.tiles());

    match game.winner().expect("A finished game should have winner") {
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }
}
