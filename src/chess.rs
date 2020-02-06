
use crate::entities::{
    GameBoard,
    PieceType,
    Move,
    Color,
};

use std::cmp::{min, max};

pub fn square_as_algebraic(square: usize) -> String {
    let rank = (square as u8 % 8 + 65) as char;
    let file = (square / 8) + 1;
    String::from(format!("{}{}", rank, file))
}

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
            PieceType:: Bishop => BishopRules::can_move(chosen_move, board),
            PieceType:: Knight => KnightRules::can_move(chosen_move, board),
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

fn horizontal_is_obstructed(origin: usize, delta_x: i32, board: GameBoard) -> bool {
    for x in 1..delta_x.abs() {
        let index = if delta_x > 0 { origin + x as usize } else { origin - x as usize };
        if board.squares[index].is_some() {
            return true
        }
    }
    false
}

fn vertical_is_obstructed(origin: usize, delta_y: i32, board: GameBoard) -> bool {
    for x in 1..delta_y.abs() {
        let index = if delta_y > 0 {
            origin + x as usize * 8
        } else {
            origin - x as usize * 8
        };
        if board.squares[index].is_some() {
            return true
        }
    }
    false
}

fn diagonal_is_obstructed(origin: usize, destination: usize, board: GameBoard) -> bool {
    let low = min(origin, destination);
    let hi = max(origin, destination);

    // The difference between two diagonal squares will divide by 7 or 9
    for n in [7,9].iter() {
        if (hi - low) % n == 0 {
            for i in (low..hi).step_by(*n) {
                if board.squares[i].is_some() { return true } 
            }
        }
    }
    return false;
}

pub struct RookRules {}

impl Moveable for RookRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);
        // Return false if the path is obstructed
        if horizontal_is_obstructed(chosen_move.origin, delta_x, board) {
            return false;
        }

        match (delta_x, delta_y) {
            (0, _) => true,
            (_, 0) => true,
            _ => false,
        }
    }

    fn can_capture(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);
        // Return false if the path is obstructed
        if horizontal_is_obstructed(chosen_move.origin, delta_x, board) {
            return false;
        }

        match (delta_x, delta_y) {
            (0, _) => true,
            (_, 0) => true,
            _ => false,
        }
    }
}

pub struct BishopRules {}

impl Moveable for BishopRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);
        if delta_x.abs() != delta_y.abs() {
            return false;
        }
        return false == diagonal_is_obstructed(chosen_move.origin, chosen_move.origin, board);
    }

    fn can_capture(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);
        if delta_x.abs() != delta_y.abs() {
            return false;
        }
        return false == diagonal_is_obstructed(chosen_move.origin, chosen_move.origin, board);
    }
}


pub struct KnightRules {}

impl Moveable for KnightRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        return match (delta_x.abs(), delta_y.abs()) {
            (1, 2) => true,
            (2, 1) => true,
            (_, _) => false,
        }

    }

    fn can_capture(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        return match (delta_x.abs(), delta_y.abs()) {
            (1, 2) => true,
            (2, 1) => true,
            (_, _) => false,
        }
    }
}


// Determine the horizontal distance between two squares
pub fn position_delta(origin: usize, destination: usize) -> (i32, i32) {
    let x = (destination as i32 % 8) - (origin as i32 % 8);
    let y = (destination as i32 / 8) - (origin as i32 / 8);
    return (x, y);
}

