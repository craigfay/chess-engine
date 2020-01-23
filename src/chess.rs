
use crate::entities::{
    GameBoard,
    PieceType,
    Move,
    Color,
};

trait Moveable {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool;
    fn can_capture(chosen_move: Move, board: GameBoard) -> bool;
}

pub struct GameRules {}

impl GameRules {
    pub fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let maybe_piece = board.squares[chosen_move.origin];
        let destination = board.squares[chosen_move.destination];

        // If there is no piece present at the chosen origin
        if maybe_piece.is_none() {
            return false
        }
        
        // If there is a piece present at the chosen destination
        if false == destination.is_none() {
            return false
        }

        match chosen_move.piece {
            PieceType:: Pawn => PawnRules::can_move(chosen_move, board),
            PieceType:: Rook => RookRules::can_move(chosen_move, board),
        }
    }
}

pub struct PawnRules {}

impl Moveable for PawnRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let piece = board.squares[chosen_move.origin].unwrap();

        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        match piece.color {
            // White Pieces can only move upwards
            Color::White => {
                match (delta_x, delta_y) {
                    (0, 1) => true,
                    (0, 2) => piece.has_moved == false,
                    _ => false,
                }
            },
            // Black pieces can only move downwards
            Color::Black => {
                match (delta_x, delta_y) {
                    (0, -1) => true,
                    (0, -2) => piece.has_moved == false,
                    _ => false,
                }
            }

        }
    }

    fn can_capture(chosen_move: Move, board: GameBoard) -> bool {
        let piece = board.squares[chosen_move.origin].unwrap();

        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        match piece.color {
            // White Pieces can only capture diagonally upwards
            Color::White => {
                match (delta_x, delta_y) {
                    (1, 1) => true,
                    (-1, 1) => true,
                    _ => false,
                }
            },
            // Black pieces can only capture diagonally downwards
            Color::Black => {
                match (delta_x, delta_y) {
                    (1, -1) => true,
                    (-1, -1) => true,
                    _ => false,
                }
            }

        }
    }
}


pub struct RookRules {}

impl Moveable for RookRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let piece = board.squares[chosen_move.origin].unwrap();
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        // Return false if the path is obstructed
        for x in chosen_move.origin..chosen_move.origin + (delta_x as usize) {
            println!("{}", x);
            if board.squares[x].is_some() {
                return false;
            }
        }

        match (delta_x, delta_y) {
            (0, _) => true,
            (_, 0) => true,
            _ => false,
        }
    }

    fn can_capture(chosen_move: Move, board: GameBoard) -> bool {
        let piece = board.squares[chosen_move.origin].unwrap();
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        match (delta_x, delta_y) {
            (0, _) => true,
            (_, 0) => true,
            _ => false,
        }
    }
}

// Determine the horizontal distance between two squares
pub fn position_delta(origin: usize, destination: usize) -> (i32, i32) {
    let x = (destination as i32 % 8) - (origin as i32 % 8);
    let y = (destination as i32 / 8) - (origin as i32 / 8);
    return (x, y);
}

