pub mod game;
pub mod io;
pub mod solver;
pub mod symmetry;

#[cfg(test)]
mod tests {
    use crate::game::{Game, PlayerKind, Winner};
    use crate::solver::Solver;
    use crate::symmetry::Symmetry;
    use std::fs;

    #[test]
    fn col_3_o_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 2).unwrap();
        game = game.make_move(2, 2, 2).unwrap();
        game = game.make_move(2, 1, 1).unwrap();
        game = game.make_move(1, 2, 1).unwrap();
        game = game.make_move(0, 1, 0).unwrap();
        game = game.make_move(0, 2, 0).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn diag_1_x_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 2).unwrap();
        game = game.make_move(0, 1, 2).unwrap();
        game = game.make_move(2, 2, 1).unwrap();
        game = game.make_move(2, 1, 1).unwrap();
        game = game.make_move(1, 1, 0).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn diag_2_x_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 2, 2).unwrap();
        game = game.make_move(0, 1, 2).unwrap();
        game = game.make_move(2, 0, 1).unwrap();
        game = game.make_move(2, 1, 1).unwrap();
        game = game.make_move(1, 1, 0).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn row_2_o_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 2).unwrap();
        game = game.make_move(1, 0, 2).unwrap();
        game = game.make_move(2, 1, 1).unwrap();
        game = game.make_move(1, 1, 1).unwrap();
        game = game.make_move(0, 2, 0).unwrap();
        game = game.make_move(1, 2, 0).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn no_tie_cappable() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 1).unwrap();
        game = game.make_move(0, 1, 1).unwrap();
        game = game.make_move(0, 2, 2).unwrap();
        game = game.make_move(2, 0, 2).unwrap();
        game = game.make_move(2, 1, 3).unwrap();
        game = game.make_move(2, 2, 3).unwrap();
        game = game.make_move(1, 0, 4).unwrap();
        game = game.make_move(1, 2, 4).unwrap();
        game = game.make_move(1, 1, 5).unwrap();
        assert_eq!(game.winner(), None);
    }

    #[test]
    fn tie_no_playable() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 5).unwrap();
        game = game.make_move(0, 1, 5).unwrap();
        game = game.make_move(0, 2, 4).unwrap();
        game = game.make_move(2, 0, 4).unwrap();
        game = game.make_move(2, 1, 3).unwrap();
        game = game.make_move(2, 2, 3).unwrap();
        game = game.make_move(1, 0, 2).unwrap();
        game = game.make_move(1, 2, 2).unwrap();
        game = game.make_move(1, 1, 1).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::Tie);
    }

    #[test]
    fn tie_no_pieces() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 0).unwrap();
        game = game.make_move(0, 1, 0).unwrap();
        game = game.make_move(0, 0, 1).unwrap();
        game = game.make_move(0, 1, 1).unwrap();
        game = game.make_move(0, 2, 2).unwrap();
        game = game.make_move(2, 0, 2).unwrap();
        game = game.make_move(2, 1, 3).unwrap();
        game = game.make_move(2, 2, 3).unwrap();
        game = game.make_move(1, 0, 4).unwrap();
        game = game.make_move(1, 2, 4).unwrap();
        game = game.make_move(1, 1, 5).unwrap();
        game = game.make_move(1, 2, 5).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::Tie);
    }

    #[test]
    fn the_one_from_the_video() {
        let mut game = Game::new();
        game = game.make_move(1, 1, 0).unwrap();
        game = game.make_move(1, 1, 2).unwrap();
        game = game.make_move(2, 2, 1).unwrap();
        game = game.make_move(2, 2, 4).unwrap();
        game = game.make_move(1, 1, 4).unwrap();
        game = game.make_move(0, 2, 3).unwrap();
        game = game.make_move(1, 2, 5).unwrap();
        game = game.make_move(1, 0, 5).unwrap();
        game = game.make_move(2, 1, 3).unwrap();
        game = game.make_move(0, 1, 1).unwrap();
        game = game.make_move(0, 1, 2).unwrap();
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn ai_wins() {
        let mut game = Game::new_with_size(4);
        let solver = Solver::new();

        game = game.make_move(1, 1, 3).unwrap();

        let (i, j, k) = solver.find_move(&game);
        game = game.make_move(i, j, k).unwrap();

        game = game.make_move(2, 2, 2).unwrap();

        let (i, j, k) = solver.find_move(&game);
        game = game.make_move(i, j, k).unwrap();

        game = game.make_move(2, 0, 1).unwrap();

        let (i, j, k) = solver.find_move(&game);
        game = game.make_move(i, j, k).unwrap();

        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn ai_plays_itself_o_wins() {
        // A small game
        let mut game = Game::new_with_size(3);
        let mut solver_x = Solver::new();
        let solver_o = Solver::new();
        solver_x.kind = PlayerKind::X;

        while !game.is_finished() {
            let (i, j, k) = solver_x.find_move(&game);
            game = game.make_move(i, j, k).unwrap();

            let (i, j, k) = solver_o.find_move(&game);
            game = game.make_move(i, j, k).unwrap();
        }

        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn rotations() {
        // o x o    0 0 3
        // x _ o    1 _ 1
        // x o x    3 2 2
        let mut game = Game::new_with_size(10);
        game = game.make_move(0, 1, 0).unwrap(); // x
        game = game.make_move(0, 0, 0).unwrap(); // o
        game = game.make_move(1, 0, 1).unwrap(); // x
        game = game.make_move(1, 2, 1).unwrap(); // o
        game = game.make_move(2, 2, 2).unwrap(); // x
        game = game.make_move(2, 1, 2).unwrap(); // o
        game = game.make_move(2, 0, 3).unwrap(); // o
        game = game.make_move(0, 2, 3).unwrap(); // o
        assert_eq!(game.winner(), None);
        assert_eq!(game.has_mirror_symmetry(Symmetry::FlipH), false);
        assert_eq!(game.has_mirror_symmetry(Symmetry::FlipV), false);
        assert_eq!(game.has_rotational_symmetry(), false);

        // x x o    3 1 0
        // o _ x    2 _ 0
        // x o o    2 1 3
        let mut game_rot90 = Game::new_with_size(10);
        game_rot90 = game_rot90.make_move(1, 2, 0).unwrap(); // x
        game_rot90 = game_rot90.make_move(0, 2, 0).unwrap(); // o
        game_rot90 = game_rot90.make_move(0, 1, 1).unwrap(); // x
        game_rot90 = game_rot90.make_move(2, 1, 1).unwrap(); // o
        game_rot90 = game_rot90.make_move(2, 0, 2).unwrap(); // x
        game_rot90 = game_rot90.make_move(1, 0, 2).unwrap(); // o
        game_rot90 = game_rot90.make_move(0, 0, 3).unwrap(); // x
        game_rot90 = game_rot90.make_move(2, 2, 3).unwrap(); // o

        assert_eq!(game.fliptate(&Symmetry::Rotate90), game_rot90);

        // x o x    2 2 3
        // o _ x    1 _ 1
        // o x o    3 0 0
        let mut game_rot180 = Game::new_with_size(10);
        game_rot180 = game_rot180.make_move(2, 1, 0).unwrap(); // x
        game_rot180 = game_rot180.make_move(2, 2, 0).unwrap(); // o
        game_rot180 = game_rot180.make_move(1, 2, 1).unwrap(); // x
        game_rot180 = game_rot180.make_move(1, 0, 1).unwrap(); // o
        game_rot180 = game_rot180.make_move(0, 0, 2).unwrap(); // x
        game_rot180 = game_rot180.make_move(0, 1, 2).unwrap(); // o
        game_rot180 = game_rot180.make_move(0, 2, 3).unwrap(); // x
        game_rot180 = game_rot180.make_move(2, 0, 3).unwrap(); // o

        assert_eq!(game.fliptate(&Symmetry::Rotate180), game_rot180);

        // x x o    3 1 2
        // o _ x    0 _ 2
        // x o o    0 1 3
        let mut game_rot270 = Game::new_with_size(10);
        game_rot270 = game_rot270.make_move(2, 1, 1).unwrap(); // x
        game_rot270 = game_rot270.make_move(0, 1, 1).unwrap(); // o
        game_rot270 = game_rot270.make_move(1, 0, 0).unwrap(); // x
        game_rot270 = game_rot270.make_move(2, 0, 0).unwrap(); // o
        game_rot270 = game_rot270.make_move(0, 2, 2).unwrap(); // x
        game_rot270 = game_rot270.make_move(1, 2, 2).unwrap(); // o
        game_rot270 = game_rot270.make_move(2, 2, 3).unwrap(); // x
        game_rot270 = game_rot270.make_move(0, 0, 3).unwrap(); // o

        assert_eq!(game.fliptate(&Symmetry::Rotate270), game_rot270);

        // x o x    3 2 2
        // x _ o    1 _ 1
        // o x o    0 0 3
        let mut game_fliph = Game::new_with_size(10);
        game_fliph = game_fliph.make_move(2, 1, 0).unwrap(); // x
        game_fliph = game_fliph.make_move(2, 0, 0).unwrap(); // o
        game_fliph = game_fliph.make_move(1, 0, 1).unwrap(); // x
        game_fliph = game_fliph.make_move(1, 2, 1).unwrap(); // o
        game_fliph = game_fliph.make_move(0, 2, 2).unwrap(); // x
        game_fliph = game_fliph.make_move(0, 1, 2).unwrap(); // o
        game_fliph = game_fliph.make_move(0, 0, 3).unwrap(); // o
        game_fliph = game_fliph.make_move(2, 2, 3).unwrap(); // o

        assert_eq!(game.fliptate(&Symmetry::FlipH), game_fliph);

        // o x o    3 0 0
        // o _ x    1 _ 1
        // x o x    2 2 3
        let mut game_flipv = Game::new_with_size(10);
        game_flipv = game_flipv.make_move(0, 1, 0).unwrap(); // x
        game_flipv = game_flipv.make_move(0, 2, 0).unwrap(); // o
        game_flipv = game_flipv.make_move(1, 2, 1).unwrap(); // x
        game_flipv = game_flipv.make_move(1, 0, 1).unwrap(); // o
        game_flipv = game_flipv.make_move(2, 0, 2).unwrap(); // x
        game_flipv = game_flipv.make_move(2, 1, 2).unwrap(); // o
        game_flipv = game_flipv.make_move(2, 2, 3).unwrap(); // x
        game_flipv = game_flipv.make_move(0, 0, 3).unwrap(); // o

        assert_eq!(game.fliptate(&Symmetry::FlipV), game_flipv);

        let mut game = Game::new();
        game = game.make_move(0, 0, 2).unwrap();
        let new_game = game.clone().fliptate(&Symmetry::Rotate90);
        assert_eq!(game, new_game.fliptate(&Symmetry::Rotate270));

        let mut game = Game::new();
        game = game.make_move(0, 0, 2).unwrap();
        let new_game = game.clone().fliptate(&Symmetry::Rotate180);
        assert_eq!(game, new_game.fliptate(&Symmetry::Rotate180));

        let mut game = Game::new();
        game = game.make_move(0, 0, 2).unwrap();
        let new_game = game.clone().fliptate(&Symmetry::FlipH);
        assert_eq!(game, new_game.fliptate(&Symmetry::FlipH));

        let mut game = Game::new();
        game = game.make_move(0, 0, 2).unwrap();
        let new_game = game.clone().fliptate(&Symmetry::FlipV);
        assert_eq!(game, new_game.fliptate(&Symmetry::FlipV));
    }

    #[test]
    fn lookup_symmetries() {
        match fs::remove_file("./data/test_lookup.json".to_string()) {
            _ => (),
        }
        let solver = Solver::new_distinct_lookup("./data/test_lookup.json".to_string());

        // Make first move in top-left corner
        let mut game = Game::new_with_size(3);
        game = game.make_move(0, 0, 2).unwrap();
        solver.find_move(&game);
        let s = solver.check_lookup(&game);
        assert!(s.is_some());

        // Check that the symmetries can be found
        let mut game = Game::new_with_size(3);
        game = game.make_move(2, 2, 2).unwrap();
        let s = solver.check_lookup(&game);
        assert!(s.is_some());

        let mut game = Game::new_with_size(3);
        game = game.make_move(0, 2, 2).unwrap();
        let s = solver.check_lookup(&game);
        assert!(s.is_some());

        let mut game = Game::new_with_size(3);
        game = game.make_move(2, 0, 2).unwrap();
        let s = solver.check_lookup(&game);
        assert!(s.is_some());

        // Make first move in top-middle
        let mut game = Game::new_with_size(3);
        game = game.make_move(0, 1, 2).unwrap();
        solver.find_move(&game);
        let s = solver.check_lookup(&game);
        assert!(s.is_some());

        // Check that the symmetries can be found
        let mut game = Game::new_with_size(3);
        game = game.make_move(1, 2, 2).unwrap();
        let s = solver.check_lookup(&game);
        assert!(s.is_some());

        let mut game = Game::new_with_size(3);
        game = game.make_move(1, 0, 2).unwrap();
        let s = solver.check_lookup(&game);
        assert!(s.is_some());

        let mut game = Game::new_with_size(3);
        game = game.make_move(2, 1, 2).unwrap();
        let s = solver.check_lookup(&game);
        assert!(s.is_some());
        match fs::remove_file("./data/test_lookup.json".to_string()) {
            _ => (),
        }
    }
}
