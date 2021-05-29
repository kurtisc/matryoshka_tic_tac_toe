use crate::game::{Game, Tile};
use strum_macros::EnumIter;

// These are ordered so they're most efficient when iterated through
#[derive(EnumIter)]
pub enum Symmetry {
    FlipH,
    FlipV,
    Rotate90,
    Rotate180,
    Rotate270,
    NoSymmetry,
}

impl Game {
    pub fn symmetry_range(&self, symmetry: Symmetry) -> Vec<usize> {
        if self.has_rotational_symmetry() {
            return (0..2).collect::<Vec<usize>>();
        } else if self.has_mirror_symmetry(symmetry) {
            return (0..2).collect::<Vec<usize>>();
        }

        (0..3).collect::<Vec<usize>>()
    }

    pub fn has_mirror_symmetry(&self, symmetry: Symmetry) -> bool {
        match symmetry {
            Symmetry::FlipH => {
                self.tiles[0][0] == self.tiles[2][0]
                    && self.tiles[0][1] == self.tiles[2][1]
                    && self.tiles[0][2] == self.tiles[2][2]
            }
            Symmetry::FlipV => {
                self.tiles[0][0] == self.tiles[0][2]
                    && self.tiles[1][0] == self.tiles[1][2]
                    && self.tiles[2][0] == self.tiles[2][2]
            }
            _ => false,
        }
    }

    pub fn has_rotational_symmetry(&self) -> bool {
        self.tiles == self.fliptate(&Symmetry::Rotate90).tiles
            && self.tiles == self.fliptate(&Symmetry::Rotate180).tiles
            && self.tiles == self.fliptate(&Symmetry::Rotate270).tiles
    }

    fn fliptate_helper(self: &Self, f: fn((usize, usize)) -> (usize, usize), i: usize) -> Tile {
        let (x, y) = f(index_to_coordinates(i));
        return self.tiles[x][y];
    }

    pub fn fliptate(self: &Self, symmetry: &Symmetry) -> Self {
        let f = match symmetry {
            Symmetry::NoSymmetry => noop,
            Symmetry::Rotate90 => rotate_ij_by_90,
            Symmetry::Rotate180 => rotate_ij_by_180,
            Symmetry::Rotate270 => rotate_ij_by_270,
            Symmetry::FlipH => flip_ij_horizontally,
            Symmetry::FlipV => flip_ij_vertically,
        };

        let mut after = self.clone();
        let flipped = (0..9)
            .collect::<Vec<usize>>()
            .iter()
            .map(|i| self.fliptate_helper(f, *i))
            .collect::<Vec<Tile>>();

        for (i, tile) in flipped.iter().enumerate() {
            let (x, y) = index_to_coordinates(i);
            after.tiles[x][y] = *tile;
        }
        after
    }
}

pub fn fliptate_ij(c: (usize, usize), symmetry: &Symmetry) -> (usize, usize) {
    let f = match symmetry {
        Symmetry::NoSymmetry => noop,
        Symmetry::Rotate90 => rotate_ij_by_90,
        Symmetry::Rotate180 => rotate_ij_by_180,
        Symmetry::Rotate270 => rotate_ij_by_270,
        Symmetry::FlipH => flip_ij_horizontally,
        Symmetry::FlipV => flip_ij_vertically,
    };
    f(c)
}

pub fn rotate_ij_by_90(c: (usize, usize)) -> (usize, usize) {
    match c {
        (0, 0) => (2, 0),
        (0, 1) => (1, 0),
        (0, 2) => (0, 0),
        (1, 0) => (2, 1),
        (1, 2) => (0, 1),
        (2, 0) => (2, 2),
        (2, 1) => (1, 2),
        (2, 2) => (0, 2),
        _ => (c),
    }
}

fn rotate_ij_by_180(c: (usize, usize)) -> (usize, usize) {
    match c {
        (0, 0) => (2, 2),
        (0, 1) => (2, 1),
        (0, 2) => (2, 0),
        (1, 0) => (1, 2),
        (1, 2) => (1, 0),
        (2, 0) => (0, 2),
        (2, 1) => (0, 1),
        (2, 2) => (0, 0),
        _ => (c),
    }
}

fn rotate_ij_by_270(c: (usize, usize)) -> (usize, usize) {
    match c {
        (0, 0) => (0, 2),
        (0, 1) => (1, 2),
        (0, 2) => (2, 2),
        (1, 0) => (0, 1),
        (1, 2) => (2, 1),
        (2, 0) => (0, 0),
        (2, 1) => (1, 0),
        (2, 2) => (2, 0),
        _ => (c),
    }
}

fn flip_ij_horizontally(c: (usize, usize)) -> (usize, usize) {
    match c {
        (0, 0) => (2, 0),
        (0, 1) => (2, 1),
        (0, 2) => (2, 2),
        (2, 0) => (0, 0),
        (2, 1) => (0, 1),
        (2, 2) => (0, 2),
        _ => (c),
    }
}

fn flip_ij_vertically(c: (usize, usize)) -> (usize, usize) {
    match c {
        (0, 0) => (0, 2),
        (0, 2) => (0, 0),
        (1, 0) => (1, 2),
        (1, 2) => (1, 0),
        (2, 0) => (2, 2),
        (2, 2) => (2, 0),
        _ => (c),
    }
}

fn noop(c: (usize, usize)) -> (usize, usize) {
    c
}

fn index_to_coordinates(i: usize) -> (usize, usize) {
    match i {
        0 => (0, 0),
        1 => (0, 1),
        2 => (0, 2),
        3 => (1, 0),
        4 => (1, 1),
        5 => (1, 2),
        6 => (2, 0),
        7 => (2, 1),
        8 => (2, 2),
        _ => (1, 1),
    }
}
