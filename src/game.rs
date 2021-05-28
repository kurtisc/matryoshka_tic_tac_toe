// Above 4, the solver will take a very long time to run
const NUMBER_OF_PIECES: usize = 6;

use smallvec::SmallVec;

#[derive(Debug, Clone)]
pub struct Player {
    pub kind: PlayerKind,
    pub pieces: SmallVec<[usize; NUMBER_OF_PIECES]>,
}

impl Player {
    pub fn new(kind: PlayerKind, pieces: SmallVec<[usize; NUMBER_OF_PIECES]>) -> Self {
        Self {
            kind: kind,
            pieces: pieces,
        }
    }

    fn remove_playable_piece(&mut self, size: usize) -> usize {
        match self.pieces.binary_search(&size) {
            Ok(i) => self.pieces.remove(i),
            Err(_) => 0,
        }
    }

    pub fn print_pieces(self) {
        println!("Pieces for {:?} {:?}", self.kind, self.pieces);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlayerKind {
    X,
    O,
}

pub type Tile = Option<(PlayerKind, usize)>;
pub type Tiles = [[Tile; 3]; 3];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Winner {
    X,
    O,
    Tie,
}

#[derive(Clone)]
pub struct Game {
    pub tiles: Tiles,
    winner: Option<Winner>,
    pub players: (Player, Player),
    current_player_kind: PlayerKind,
}

impl Game {
    pub fn new() -> Self {
        Self {
            tiles: Default::default(),
            winner: None,
            players: (
                Player::new(
                    PlayerKind::X,
                    (0..NUMBER_OF_PIECES).collect::<SmallVec<[usize; NUMBER_OF_PIECES]>>(),
                ),
                Player::new(
                    PlayerKind::O,
                    (0..NUMBER_OF_PIECES).collect::<SmallVec<[usize; NUMBER_OF_PIECES]>>(),
                ),
            ),
            current_player_kind: PlayerKind::X,
        }
    }

    pub fn new_with_size(size: usize) -> Self {
        Self {
            tiles: Default::default(),
            winner: None,
            players: (
                Player::new(
                    PlayerKind::X,
                    (0..size).collect::<SmallVec<[usize; NUMBER_OF_PIECES]>>(),
                ),
                Player::new(
                    PlayerKind::O,
                    (0..size).collect::<SmallVec<[usize; NUMBER_OF_PIECES]>>(),
                ),
            ),
            current_player_kind: PlayerKind::X,
        }
    }

    pub fn make_move(mut self, row: usize, col: usize, size: usize) -> Result<Game, &'static str> {
        if let Some(other_tile) = self.tiles[row][col] {
            let (_, other_size) = other_tile;
            if other_size >= size {
                return Err("The tile already has a bigger piece in it!");
            }
        }

        let (mut x, mut o) = self.players;

        let playable_piece = match self.current_player_kind {
            PlayerKind::X => x.remove_playable_piece(size),
            _ => o.remove_playable_piece(size),
        };

        if playable_piece != size {
            return Err("You don't have a tile with that size");
        }

        self.tiles[row][col] = Some((self.current_player_kind, size));

        self.players = (x, o);

        self.current_player_kind = match self.current_player_kind {
            PlayerKind::X => PlayerKind::O,
            PlayerKind::O => PlayerKind::X,
        };

        self.update_winner(row, col);

        Ok(self)
    }

    fn update_winner(&mut self, row: usize, col: usize) {
        let rows = self.tiles.len();

        let tiles_row = self.tiles[row];

        let tiles_col = [self.tiles[0][col], self.tiles[1][col], self.tiles[2][col]];

        let tiles_diagonal_1 = if row == col {
            [self.tiles[0][0], self.tiles[1][1], self.tiles[2][2]]
        } else {
            [None, None, None]
        };

        let tiles_diagonal_2 = if (rows - row - 1) == col {
            [self.tiles[0][2], self.tiles[1][1], self.tiles[2][0]]
        } else {
            [None, None, None]
        };

        fn check_winner(row: &[Tile]) -> Option<Winner> {
            let mut a = None;
            let mut b = None;
            let mut c = None;

            if row[0].is_some() {
                let (a_, _) = row[0].unwrap();
                a = Some(a_);
            }

            if row[1].is_some() {
                let (b_, _) = row[1].unwrap();
                b = Some(b_);
            }

            if row[2].is_some() {
                let (c_, _) = row[2].unwrap();
                c = Some(c_);
            }

            if a == b && b == c {
                match a {
                    Some(PlayerKind::X) => Some(Winner::X),
                    Some(PlayerKind::O) => Some(Winner::O),
                    None => None,
                }
            } else {
                None
            }
        }
        self.winner = self
            .winner
            .or_else(|| check_winner(&tiles_row))
            .or_else(|| check_winner(&tiles_col))
            .or_else(|| check_winner(&tiles_diagonal_1))
            .or_else(|| check_winner(&tiles_diagonal_2));

        self.winner = self.winner.or_else(|| {
            let (a, b) = self.players.clone();
            if a.pieces.is_empty() && b.pieces.is_empty() {
                Some(Winner::Tie)
            } else {
                None
            }
        });

        self.winner = self.winner.or_else(|| {
            if self
                .tiles
                .iter()
                .all(|row| row.iter().all(|tile| tile.is_some()))
            {
                self.check_cappable()
            } else {
                None
            }
        });
    }

    fn check_cappable(&self) -> Option<Winner> {
        let (x, o) = self.players.clone();

        for (_, piece) in x.pieces.iter().enumerate() {
            if self.piece_can_be_placed(piece) {
                return None;
            }
        }

        for (_, piece) in o.pieces.iter().enumerate() {
            if self.piece_can_be_placed(piece) {
                return None;
            }
        }

        Some(Winner::Tie)
    }

    pub fn piece_can_be_placed_at(&self, piece: &usize, i: usize, j: usize) -> bool {
        let (x, o) = &self.players;

        if self.current_player_kind == PlayerKind::X {
            match x.pieces.binary_search(&piece) {
                Ok(_) => (),
                Err(_) => return false,
            }
        } else {
            match o.pieces.binary_search(&piece) {
                Ok(_) => (),
                Err(_) => return false,
            }
        }

        match self.tiles[i][j] {
            Some((_, x)) => piece > &x,
            _ => true,
        }
    }

    pub fn piece_can_be_placed(&self, piece: &usize) -> bool {
        for (_, row) in self.tiles.iter().enumerate() {
            for tile in row {
                match tile {
                    Some(other_piece) => {
                        let (_, other_size) = other_piece;
                        if other_size < piece {
                            return true;
                        }
                    }
                    _ => (),
                }
            }
        }
        false
    }

    pub fn is_finished(&self) -> bool {
        self.winner.is_some()
    }

    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }

    pub fn tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub fn current_player_kind(&self) -> PlayerKind {
        self.current_player_kind
    }

    pub fn symmetry_range(&self, axis: Axis) -> Vec<usize> {
        if self.has_rotational_symmetry() {
            return (0..2).collect::<Vec<usize>>();
        } else if self.has_mirror_symmetry(axis) {
            return (0..2).collect::<Vec<usize>>();
        }

        (0..3).collect::<Vec<usize>>()
    }

    pub fn has_mirror_symmetry(&self, axis: Axis) -> bool {
        match axis {
            Axis::Horizontal => {
                if self.tiles[0][0] == self.tiles[2][0]
                    && self.tiles[0][1] == self.tiles[2][1]
                    && self.tiles[0][2] == self.tiles[2][2]
                {
                    return true;
                }
            }
            Axis::Vertical => {
                if self.tiles[0][0] == self.tiles[0][2]
                    && self.tiles[1][0] == self.tiles[1][2]
                    && self.tiles[2][0] == self.tiles[2][2]
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn has_rotational_symmetry(&self) -> bool {
        if self.tiles[0][0] == self.tiles[0][2]
            && self.tiles[0][2] == self.tiles[2][2]
            && self.tiles[2][2] == self.tiles[2][0]
            && self.tiles[0][1] == self.tiles[1][0]
            && self.tiles[1][0] == self.tiles[1][1]
            && self.tiles[1][1] == self.tiles[0][1]
        {
            return true;
        }

        false
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMove(pub String);
