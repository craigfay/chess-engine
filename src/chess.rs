
use crate::entities::{
    GameBoard,
    Piece,
    ActionType,
    PieceType,
    Move,
    Color,
};

pub struct GameRules {}

impl GameRules {
    pub fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let maybe_piece = board.squares[chosen_move.origin];

        // If there is no piece present at the chosen origin
        if maybe_piece.is_none() {
            return false
        }

        match chosen_move.piece {
            PieceType:: Pawn => PawnRules::can_move(chosen_move, board),
        }
    }
}

pub struct PawnRules {}

impl PawnRules {
    pub fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let piece = board.squares[chosen_move.origin].unwrap();

        // Pawns can only move vertically
        if 0 < delta_x(chosen_move.origin, chosen_move.destination) {
            return false
        }

        match piece.color {
            // White Pieces can only move upwards
            Color::White => {
                match delta_y(chosen_move.origin, chosen_move.destination) {
                    1 => true,
                    2 => piece.has_moved == false,
                    _ => false,
                }
            },
            // Black pieces can only move downwards
            Color::Black => {
                match delta_y(chosen_move.origin, chosen_move.destination) {
                    -1 => true,
                    -2 => piece.has_moved == false,
                    _ => false,
                }
            }

        }
    }
}

// Determine the horizontal distance between two squares
pub fn delta_x(origin: usize, destination: usize) -> i32 {
    (destination as i32 % 8) - (origin as i32 % 8)
}

// Determine the vertical distance between two squares
pub fn delta_y(origin: usize, destination: usize) -> i32 {
    (destination as i32 / 8) - (origin as i32 / 8)
}

