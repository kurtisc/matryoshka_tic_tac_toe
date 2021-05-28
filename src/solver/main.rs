// Above 4, the solver will take a very long time to run
const SOLVER_NUMBER_OF_PIECES: usize = 4;

use matryoshka_tic_tac_toe::{game, io};

use game::{Game, PlayerKind, Winner};
use io::{print_tiles, prompt_move};

fn main() {
    let mut game = Game::new_with_size(SOLVER_NUMBER_OF_PIECES);

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
            PlayerKind::O => get_min_max_move(game.clone()),
        };

        game = game.make_move(row, col, size);
    }

    print_tiles(game.tiles());

    match game.winner().expect("A finished game should have winner") {
        Winner::X => println!("x wins!"),
        Winner::O => println!("o wins!"),
        Winner::Tie => println!("Tie!"),
    }
}

fn get_min_max_move(game: Game) -> (usize, usize, usize) {
    let mut best_score = std::i64::MIN;
    let mut best_move = (0, 0, 0);
    let (_, o) = game.clone().players;

    for i in 0..3 {
        for j in 0..3 {
            for (_, piece) in o.pieces.iter().enumerate() {
                if game.piece_can_be_placed_at(piece, i, j) {
                    let new_game = game.clone().make_move(i, j, *piece);
                    let tmp_score = min_search(new_game, std::i64::MIN, std::i64::MAX);

                    if tmp_score > best_score {
                        best_score = tmp_score;
                        best_move = (i, j, *piece);
                    }
                }
            }
        }
    }

    best_move
}

fn min_search(game: Game, alpha: i64, mut beta: i64) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    let (x, _) = game.clone().players;
    for i in 0..3 {
        for j in 0..3 {
            for (_, piece) in x.pieces.iter().enumerate() {
                if game.piece_can_be_placed_at(piece, i, j) {
                    let new_game = game.clone().make_move(i, j, *piece);
                    let score = max_search(new_game, alpha, beta);

                    if score < beta {
                        beta = score;
                    }
                    if alpha > beta {
                        return beta;
                    }
                }
            }
        }
    }
    beta
}

fn max_search(game: Game, mut alpha: i64, beta: i64) -> i64 {
    match game.winner() {
        Some(Winner::X) => return -10,
        Some(Winner::O) => return 10,
        Some(Winner::Tie) => return 0,
        _ => (),
    }

    let (_, o) = game.clone().players;
    for i in 0..3 {
        for j in 0..3 {
            for (_, piece) in o.pieces.iter().enumerate() {
                if game.piece_can_be_placed_at(piece, i, j) {
                    let new_game = game.clone().make_move(i, j, *piece);
                    let score = min_search(new_game, alpha, beta);

                    if score > alpha {
                        alpha = score;
                    }
                    if alpha > beta {
                        return alpha;
                    }
                }
            }
        }
    }
    alpha
}
