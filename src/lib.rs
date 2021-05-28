pub mod game;
pub mod io;

#[cfg(test)]
mod tests {
    use crate::game::{Game, Winner};

    #[test]
    fn col_3_o_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 2);
        game = game.make_move(2, 2, 2);
        game = game.make_move(2, 1, 1);
        game = game.make_move(1, 2, 1);
        game = game.make_move(0, 1, 0);
        game = game.make_move(0, 2, 0);
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn diag_1_x_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 2);
        game = game.make_move(0, 1, 2);
        game = game.make_move(2, 2, 1);
        game = game.make_move(2, 1, 1);
        game = game.make_move(1, 1, 1);
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn diag_2_x_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 2, 2);
        game = game.make_move(0, 1, 2);
        game = game.make_move(2, 0, 1);
        game = game.make_move(2, 1, 1);
        game = game.make_move(1, 1, 0);
        assert_eq!(game.winner().unwrap(), Winner::X);
    }

    #[test]
    fn row_2_o_wins() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 2);
        game = game.make_move(1, 0, 2);
        game = game.make_move(2, 1, 1);
        game = game.make_move(1, 1, 1);
        game = game.make_move(0, 2, 0);
        game = game.make_move(1, 2, 0);
        assert_eq!(game.winner().unwrap(), Winner::O);
    }

    #[test]
    fn no_tie_cappable() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 1);
        game = game.make_move(0, 1, 1);
        game = game.make_move(0, 2, 2);
        game = game.make_move(2, 0, 2);
        game = game.make_move(2, 1, 3);
        game = game.make_move(2, 2, 3);
        game = game.make_move(1, 0, 4);
        game = game.make_move(1, 2, 4);
        game = game.make_move(1, 1, 5);
        assert_eq!(game.winner(), None);
    }

    #[test]
    fn tie_no_playable() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 5);
        game = game.make_move(0, 1, 5);
        game = game.make_move(0, 2, 4);
        game = game.make_move(2, 0, 4);
        game = game.make_move(2, 1, 3);
        game = game.make_move(2, 2, 3);
        game = game.make_move(1, 0, 2);
        game = game.make_move(1, 2, 2);
        game = game.make_move(1, 1, 1);
        assert_eq!(game.winner().unwrap(), Winner::Tie);
    }

    #[test]
    fn tie_no_pieces() {
        let mut game = Game::new();
        game = game.make_move(0, 0, 0);
        game = game.make_move(0, 1, 0);
        game = game.make_move(0, 0, 1);
        game = game.make_move(0, 1, 1);
        game = game.make_move(0, 2, 2);
        game = game.make_move(2, 0, 2);
        game = game.make_move(2, 1, 3);
        game = game.make_move(2, 2, 3);
        game = game.make_move(1, 0, 4);
        game = game.make_move(1, 2, 4);
        game = game.make_move(1, 1, 5);
        game = game.make_move(1, 2, 5);
        assert_eq!(game.winner().unwrap(), Winner::Tie);
    }

    #[test]
    fn the_one_from_the_video() {
        let mut game = Game::new();
        game = game.make_move(1, 1, 0);
        game = game.make_move(1, 1, 2);
        game = game.make_move(2, 2, 1);
        game = game.make_move(2, 2, 4);
        game = game.make_move(1, 1, 4);
        game = game.make_move(0, 2, 3);
        game = game.make_move(1, 2, 5);
        game = game.make_move(1, 0, 5);
        game = game.make_move(2, 1, 3);
        game = game.make_move(0, 1, 1);
        game = game.make_move(0, 1, 2);
        assert_eq!(game.winner().unwrap(), Winner::X);
    }
}
