pub mod game;
pub mod io;
pub mod solver;

#[cfg(test)]
mod tests {
    use crate::solver::Solver;

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
    fn ai_wins_game() {
        let mut game = Game::new_with_size(3);
        let solver = Solver::new();

        game = game.make_move(0, 0, 2).unwrap();
        game = game.make_move(2, 2, 2).unwrap();
        game = game.make_move(2, 1, 1).unwrap();
        game = game.make_move(1, 2, 1).unwrap();
        game = game.make_move(0, 1, 0).unwrap();

        let (_, b) = solver.find_move(&game);
        let (i, j, k) = b;
        game = game.make_move(i, j, k).unwrap();

        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn ai_avoids_loss() {
        let mut game = Game::new_with_size(3);
        let mut solver = Solver::new();

        game = game.make_move(2, 1, 0).unwrap();

        let (a, b) = solver.find_move(&game);
        solver = a;
        let (i, j, k) = b;
        game = game.make_move(i, j, k).unwrap();

        game = game.make_move(2, 0, 1).unwrap();

        let (a, b) = solver.find_move(&game);
        solver = a;
        let (i, j, k) = b;
        game = game.make_move(i, j, k).unwrap();

        game = game.make_move(2, 2, 2).unwrap();

        let (_, b) = solver.find_move(&game);
        let (i, j, k) = b;
        game = game.make_move(i, j, k).unwrap();

        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn ai_ideal_first_move() {
        let mut game = Game::new_with_size(3);
        let solver = Solver::new();

        game = game.make_move(2, 1, 0).unwrap();

        let (_, b) = solver.find_move(&game);
        let (i, j, k) = b;
        game = game.make_move(i, j, k).unwrap();

        assert!(game.tiles[1][1] != None);
    }
}
