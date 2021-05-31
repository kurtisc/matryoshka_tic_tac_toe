use crate::game::{Game, Tile};
use std::convert::TryInto;
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

impl Symmetry {
    pub fn reverse(self: Self) -> Symmetry {
        match self {
            Symmetry::Rotate90 => Symmetry::Rotate270,
            Symmetry::Rotate270 => Symmetry::Rotate90,
            _ => self,
        }
    }
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

    pub fn fliptate(self: &Self, symmetry: &Symmetry) -> Self {
        let f = match symmetry {
            Symmetry::NoSymmetry => noop,
            Symmetry::Rotate90 => rotate_coordinates_by_90,
            Symmetry::Rotate180 => rotate_coordinates_by_180,
            Symmetry::Rotate270 => rotate_coordinates_by_270,
            Symmetry::FlipH => flip_coordinates_horizontally,
            Symmetry::FlipV => flip_coordinates_vertically,
        };

        let mut after = self.clone();
        let flipped = (0..9)
            .collect::<Vec<usize>>()
            .iter()
            .map(|i| self.tiles.data[f(*i)])
            .collect::<Vec<Tile>>();

        after.tiles.data = flipped.try_into().unwrap();
        after
    }
}

pub fn fliptate_coordinates(c: usize, symmetry: &Symmetry) -> usize {
    let f = match symmetry {
        Symmetry::NoSymmetry => noop,
        Symmetry::Rotate90 => rotate_coordinates_by_90,
        Symmetry::Rotate180 => rotate_coordinates_by_180,
        Symmetry::Rotate270 => rotate_coordinates_by_270,
        Symmetry::FlipH => flip_coordinates_horizontally,
        Symmetry::FlipV => flip_coordinates_vertically,
    };
    f(c)
}

pub fn rotate_coordinates_by_90(c: usize) -> usize {
    match c {
        0 => 6,
        1 => 3,
        2 => 0,
        3 => 7,
        5 => 1,
        6 => 8,
        7 => 5,
        8 => 2,
        _ => (c),
    }
}

fn rotate_coordinates_by_180(c: usize) -> usize {
    match c {
        0 => 8,
        1 => 7,
        2 => 6,
        3 => 5,
        5 => 3,
        6 => 2,
        7 => 1,
        8 => 0,
        _ => (c),
    }
}

fn rotate_coordinates_by_270(c: usize) -> usize {
    match c {
        0 => 2,
        1 => 5,
        2 => 8,
        3 => 1,
        5 => 7,
        6 => 0,
        7 => 3,
        8 => 6,
        _ => (c),
    }
}

fn flip_coordinates_horizontally(c: usize) -> usize {
    match c {
        0 => 6,
        1 => 7,
        2 => 8,
        6 => 0,
        7 => 1,
        8 => 2,
        _ => (c),
    }
}

fn flip_coordinates_vertically(c: usize) -> usize {
    match c {
        0 => 2,
        2 => 0,
        3 => 5,
        5 => 3,
        6 => 8,
        8 => 6,
        _ => (c),
    }
}

fn noop(c: usize) -> usize {
    c
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

pub fn coordinates_to_index(c: (usize, usize)) -> usize {
    match c {
        (0, 0) => 0,
        (0, 1) => 1,
        (0, 2) => 2,
        (1, 0) => 3,
        (1, 1) => 4,
        (1, 2) => 5,
        (2, 0) => 6,
        (2, 1) => 7,
        (2, 2) => 8,
        _ => 0,
    }
}

pub fn index_to_coordinates(i: usize) -> (usize, usize) {
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
