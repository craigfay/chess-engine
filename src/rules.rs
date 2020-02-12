
use crate::entities::{
    GameBoard,
    PieceType,
    Move,
    Color,
};

use crate::notation::{algebraic};

use std::cmp::{min, max};

trait Moveable {
    fn is_legal(chosen_move: Move, board: GameBoard) -> bool;
}

pub fn is_legal(chosen_move: Move, board: GameBoard) -> bool {
    GameRules::is_legal(chosen_move, board)
}


pub struct GameRules {}

impl GameRules {
    pub fn is_legal(chosen_move: Move, board: GameBoard) -> bool {
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
            PieceType:: Pawn => PawnRules::is_legal(chosen_move, board),
            PieceType:: Rook => RookRules::is_legal(chosen_move, board),
            PieceType:: Bishop => BishopRules::is_legal(chosen_move, board),
            PieceType:: Knight => KnightRules::is_legal(chosen_move, board),
            PieceType:: Queen => QueenRules::is_legal(chosen_move, board),
            PieceType:: King => KingRules::is_legal(chosen_move, board),
        }
    }
}

pub struct PawnRules {}

impl Moveable for PawnRules {
    fn is_legal(chosen_move: Move, board: GameBoard) -> bool {
        let piece = board.squares[chosen_move.origin].unwrap();

        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        let destination_is_enemy_piece = match board.squares[chosen_move.destination] {
            Some(other_piece) => other_piece.color != piece.color,
            None => false,
        };

        match piece.color {
            // White Pieces can only move upwards
            Color::White => {
                match (delta_x, delta_y) {
                    (0, 1) => true,
                    (0, 2) => piece.has_moved == false,
                    // Captures
                    (1, 1) => destination_is_enemy_piece,
                    (-1, 1) => destination_is_enemy_piece,
                    
                    _ => false,
                }
            },
            // Black pieces can only move downwards
            Color::Black => {
                match (delta_x, delta_y) {
                    (0, -1) => true,
                    (0, -2) => piece.has_moved == false,
                    // Captures
                    (1, -1) => destination_is_enemy_piece,
                    (-1, -1) => destination_is_enemy_piece,
                    _ => false,
                }
            }

        }
    }
}

fn horizontal_path_is_obstructed(origin: usize, delta_x: i32, board: GameBoard) -> bool {
    for x in 1..delta_x.abs() {
        let index = if delta_x > 0 { origin + x as usize } else { origin - x as usize };
        if board.squares[index].is_some() {
            return true
        }
    }
    false
}

fn vertical_path_is_obstructed(origin: usize, delta_y: i32, board: GameBoard) -> bool {
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
    fn is_legal(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);
        // Return false if the path is obstructed
        if horizontal_path_is_obstructed(chosen_move.origin, delta_x, board) {
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
    fn is_legal(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);
        if delta_x.abs() != delta_y.abs() {
            return false;
        }
        return false == diagonal_is_obstructed(chosen_move.origin, chosen_move.origin, board);
    }
}


pub struct KnightRules {}

impl Moveable for KnightRules {
    fn is_legal(chosen_move: Move, _board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        return match (delta_x.abs(), delta_y.abs()) {
            (1, 2) => true,
            (2, 1) => true,
            (_, _) => false,
        }

    }
}

pub struct QueenRules {}

impl Moveable for QueenRules {
    fn is_legal(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        return match (delta_x.abs(), delta_y.abs()) {
            (0, _) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (_, 0) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (x, y) => x == y && !diagonal_is_obstructed(chosen_move.origin, chosen_move.origin, board),
        }

    }
}

pub struct KingRules {}

impl Moveable for KingRules {
    fn is_legal(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        return match (delta_x.abs(), delta_y.abs()) {
            (0, 1) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (1, 0) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (1, 1) => !diagonal_is_obstructed(chosen_move.origin, chosen_move.origin, board),
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

