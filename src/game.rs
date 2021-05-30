const NUMBER_OF_PIECES: usize = 6;

use core::cmp::max;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Player {
    pub kind: PlayerKind,
    pub pieces: Vec<usize>,
}

impl Player {
    pub fn new(kind: PlayerKind, pieces: Vec<usize>) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerKind {
    X,
    O,
}

pub type Tile = Option<(PlayerKind, usize)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tiles {
    pub data: [Tile; 9],
}

impl Index<usize> for Tiles {
    type Output = [Tile];
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * 3..(index + 1) * 3]
    }
}

impl IndexMut<usize> for Tiles {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * 3..(index + 1) * 3]
    }
}

impl Tiles {
    pub fn data(&self) -> [Tile; 9] {
        self.data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Winner {
    X,
    O,
    Tie,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct Game {
    pub tiles: Tiles,
    winner: Option<Winner>,
    pub players: (Player, Player),
    current_player_kind: PlayerKind,
}

impl Game {
    pub fn new() -> Self {
        Self {
            tiles: Tiles { data: [None; 9] },
            winner: None,
            players: (
                Player::new(PlayerKind::X, (0..NUMBER_OF_PIECES).collect::<Vec<usize>>()),
                Player::new(PlayerKind::O, (0..NUMBER_OF_PIECES).collect::<Vec<usize>>()),
            ),
            current_player_kind: PlayerKind::X,
        }
    }

    pub fn new_with_size(size: usize) -> Self {
        Self {
            tiles: Tiles { data: [None; 9] },
            winner: None,
            players: (
                Player::new(PlayerKind::X, (0..size).collect::<Vec<usize>>()),
                Player::new(PlayerKind::O, (0..size).collect::<Vec<usize>>()),
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
        let rows = 3;

        let tiles_row = &self.tiles[row];

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
            if self.tiles.data.iter().all(|tile| tile.is_some()) {
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

    pub fn piece_can_be_placed(&self, piece: &usize) -> bool {
        for tile in self.tiles.data.iter() {
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

    pub fn get_number_of_pieces(&self) -> usize {
        1 + self.get_biggest_piece()
    }

    pub fn get_biggest_piece(&self) -> usize {
        let (x, o) = self.players.clone();
        let mut biggest_placed_piece = 0;
        let tiles = self.tiles.data.iter();
        for tile in tiles {
            match tile {
                Some((_, size)) => {
                    if *size > biggest_placed_piece {
                        biggest_placed_piece = *size;
                    }
                }
                _ => (),
            }
        }
        max(
            biggest_placed_piece,
            *max(o.pieces.iter().max(), x.pieces.iter().max()).unwrap(),
        )
    }

    pub fn get_turn_count(&self) -> usize {
        let (x, o) = self.players.clone();
        let max_turns = 2 * self.get_number_of_pieces();

        let remaining_turns = x.pieces.len() + o.pieces.len();
        (max_turns - remaining_turns) + 1
    }
}

#[derive(Debug, Clone)]
pub struct InvalidMove(pub String);
