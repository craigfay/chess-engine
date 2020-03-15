// The single responsibility of this module is to define data structures
// that represent real world chess objects and game states.

use crate::gamestate::GameState;

#[derive(Debug)]
pub struct Placement {
    pub color: Color,
    pub piece: PieceName,
    pub square: usize,
}

impl Placement {
    pub  fn new(color: Color, piece: PieceName, square: usize) -> Placement {
        Placement { color, piece, square }
    }
}

pub trait Action {
    fn is_legal(&self, state: &GameState) -> bool;
    fn apply(&self, state: &GameState) -> GameState;
    fn name(&self) -> &str;
    fn as_algebraic_notation(&self, state: &GameState) -> String;
}

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

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum CastleDirection {
    Kingside,
    Queenside,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub struct Castle {
    pub direction: CastleDirection,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Promotion {
    pub pawn_becomes: PieceName,
    pub moving_from: usize,
    pub to: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Capture {
    pub on: usize,
    pub with: usize,
}



#[derive(Debug)]
#[derive(PartialEq)]
pub struct EnPassant {
    pub with: usize,
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

