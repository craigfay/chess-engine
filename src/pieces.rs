// The single responsibility of this module is to define data structures
// that represent real world chess objects and game states.

use crate::gamestate::GameState;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PieceName {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Piece {
    pub color: Color,
    pub name: PieceName,
}

impl Piece {
    pub fn new(color: Color, name: PieceName) -> Piece {
        Piece { color, name }
    }
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        match self.name {
            PieceName::Pawn => String::from(""),
            PieceName::Bishop => String::from("B"),
            PieceName::Knight => String::from("N"),
            PieceName::Rook => String::from("R"),
            PieceName::Queen => String::from("Q"),
            PieceName::King => String::from("K"),
        }
    }
}

