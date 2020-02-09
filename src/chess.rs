
use crate::entities::{
    GameBoard,
    PieceType,
    Move,
    Color,
};

use std::cmp::{min, max};

pub fn index_as_algebraic(square: usize) -> String {
    let rank = (square as u8 % 8 + 97) as char;
    let file = (square / 8) + 1;
    String::from(format!("{}{}", rank, file))
}


pub fn algebraic(s: &str) -> Option<usize> {
    match s {
        "a1" => Some(0),
        "b1" => Some(1),
        "c1" => Some(2),
        "d1" => Some(3),
        "e1" => Some(4),
        "f1" => Some(5),
        "g1" => Some(6),
        "h1" => Some(7),
        "a2" => Some(8),
        "b2" => Some(9),
        "c2" => Some(10),
        "d2" => Some(11),
        "e2" => Some(12),
        "f2" => Some(13),
        "g2" => Some(14),
        "h2" => Some(15),
        "a3" => Some(16),
        "b3" => Some(17),
        "c3" => Some(18),
        "d3" => Some(19),
        "e3" => Some(20),
        "f3" => Some(21),
        "g3" => Some(22),
        "h3" => Some(23),
        "a4" => Some(24),
        "b4" => Some(25),
        "c4" => Some(26),
        "d4" => Some(27),
        "e4" => Some(28),
        "f4" => Some(29),
        "g4" => Some(30),
        "h4" => Some(31),
        "a5" => Some(32),
        "b5" => Some(33),
        "c5" => Some(34),
        "d5" => Some(35),
        "e5" => Some(36),
        "f5" => Some(37),
        "g5" => Some(38),
        "h5" => Some(39),
        "a6" => Some(40),
        "b6" => Some(41),
        "c6" => Some(42),
        "d6" => Some(43),
        "e6" => Some(44),
        "f6" => Some(45),
        "g6" => Some(46),
        "h6" => Some(47),
        "a7" => Some(48),
        "b7" => Some(49),
        "c7" => Some(50),
        "d7" => Some(51),
        "e7" => Some(52),
        "f7" => Some(53),
        "g7" => Some(54),
        "h7" => Some(55),
        "a8" => Some(55),
        "b8" => Some(57),
        "c8" => Some(58),
        "d8" => Some(59),
        "e8" => Some(60),
        "f8" => Some(61),
        "g8" => Some(62),
        "h8" => Some(63),
        _ => None
    }
}

pub fn algebraic_as_index(s: &str) -> Option<usize> {
    let mut chars = s.chars();
    let file = chars.next();
    let rank = chars.next();
    let end = chars.next();

    if !rank.is_some() { return None }
    if !file.is_some() { return None }
    if end.is_some() { return None }

    if !file.unwrap().is_alphabetic() { return None }
    if !rank.unwrap().is_numeric() { return None }

    let file = file.unwrap() as usize - 97;
    let rank = (rank.unwrap().to_digit(10).unwrap() as usize - 1) * 8;

    Some(file + rank)

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
            PieceType:: Queen => QueenRules::can_move(chosen_move, board),
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
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
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

    fn can_capture(chosen_move: Move, board: GameBoard) -> bool {
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

pub struct QueenRules {}

impl Moveable for QueenRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        return match (delta_x.abs(), delta_y.abs()) {
            (0, _) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (_, 0) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (x, y) => x == y && !diagonal_is_obstructed(chosen_move.origin, chosen_move.origin, board),
            _ => false,
        }

    }

    fn can_capture(chosen_move: Move, board: GameBoard) -> bool {
        let (delta_x, delta_y)  = position_delta(chosen_move.origin, chosen_move.destination);

        return match (delta_x.abs(), delta_y.abs()) {
            (0, _) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (_, 0) => !horizontal_path_is_obstructed(chosen_move.origin, delta_x, board),
            (x, y) => x == y && !diagonal_is_obstructed(chosen_move.origin, chosen_move.origin, board),
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

