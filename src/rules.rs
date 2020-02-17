// The single responsibility of this module is to determine what type of actions
// are legal to perform on an arbitrary GameState.

use crate::entities::{
    GameState,
    PieceName,
    Move,
    Color::{White, Black},
};

use crate::notation::{algebraic};

use std::cmp::{min, max};

pub fn legal_moves(state: &GameState) -> Vec<Move> {
    let mut results = vec![];

    for origin in 0..64 {
        if state.squares[origin].is_some() {
            let piece = state.squares[origin].unwrap();
            if piece.color == state.to_move {
                for destination in 0..64 {
                    let m = Move { origin, destination, piece: piece.name };;
                    if move_is_legal(&m, state) {
                        results.push(m);
                    }
                }
            }

        }
    }
    results
}

pub fn move_is_legal(m: &Move, state: &GameState) -> bool {
    let maybe_piece = state.squares[m.origin];
    let destination = state.squares[m.destination];

    // If there is no piece present at the chosen origin
    if maybe_piece.is_none() {
        return false
    }
    
    // If there is a piece present at the chosen destination
    if false == destination.is_none() {
        return false
    }

    match m.piece {
        PieceName:: Pawn => pawn_move_is_legal(m, state),
        PieceName:: Rook => rook_move_is_legal(m, state),
        PieceName:: Bishop => bishop_move_is_legal(m, state),
        PieceName:: Knight => knight_move_is_legal(m, state),
        PieceName:: Queen => queen_move_is_legal(m, state),
        PieceName:: King => king_move_is_legal(m, state),
    }
}


fn pawn_move_is_legal(m: &Move, state: &GameState) -> bool {
    let piece = state.squares[m.origin].unwrap();

    let (delta_x, delta_y)  = position_delta(m.origin, m.destination);

    let destination_is_enemy_piece = match state.squares[m.destination] {
        Some(other_piece) => other_piece.color != piece.color,
        None => false,
    };

    match piece.color {
        // White Pieces can only move upwards
        White => {
            match (delta_x, delta_y) {
                // Normal Moves
                (0, 1) => true,
                // Two-Square Moves
                (0, 2) => m.origin > 7 && m.origin < 16,
                // Captures
                (1, 1) => destination_is_enemy_piece,
                (-1, 1) => destination_is_enemy_piece,
                _ => false,
            }
        },
        // Black pieces can only move downwards
        Black => {
            match (delta_x, delta_y) {
                // Normal Moves
                (0, -1) => true,
                // Two-Square Moves
                (0, -2) => m.origin > 47 && m.origin < 56,
                // Captures
                (1, -1) => destination_is_enemy_piece,
                (-1, -1) => destination_is_enemy_piece,
                _ => false,
            }
        }

    }
}

fn horizontal_path_is_obstructed(origin: usize, delta_x: i32, state: &GameState) -> bool {
    for x in 1..delta_x.abs() {
        let index = if delta_x > 0 { origin + x as usize } else { origin - x as usize };
        if state.squares[index].is_some() {
            return true
        }
    }
    false
}

fn vertical_path_is_obstructed(origin: usize, delta_y: i32, state: &GameState) -> bool {
    for x in 1..delta_y.abs() {
        let index = if delta_y > 0 {
            origin + x as usize * 8
        } else {
            origin - x as usize * 8
        };
        if state.squares[index].is_some() {
            return true
        }
    }
    false
}

fn diagonal_is_obstructed(origin: usize, destination: usize, state: &GameState) -> bool {
    let low = min(origin, destination);
    let hi = max(origin, destination);

    // The difference between two diagonal squares will divide by 7 or 9
    for n in [7,9].iter() {
        if (hi - low) % n == 0 {
            for i in (low..hi).step_by(*n) {
                if state.squares[i].is_some() { return true } 
            }
        }
    }
    return false;
}

fn rook_move_is_legal(m: &Move, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.origin, m.destination);
    // Return false if the path is obstructed
    if horizontal_path_is_obstructed(m.origin, delta_x, state) {
        return false;
    }

    match (delta_x, delta_y) {
        (0, _) => true,
        (_, 0) => true,
        _ => false,
    }
}

fn bishop_move_is_legal(m: &Move, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.origin, m.destination);
    if delta_x.abs() != delta_y.abs() {
        return false;
    }
    return false == diagonal_is_obstructed(m.origin, m.origin, state);
}


fn knight_move_is_legal(m: &Move, _state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.origin, m.destination);

    return match (delta_x.abs(), delta_y.abs()) {
        (1, 2) => true,
        (2, 1) => true,
        (_, _) => false,
    }

}

fn queen_move_is_legal(m: &Move, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.origin, m.destination);

    return match (delta_x.abs(), delta_y.abs()) {
        (0, _) => !horizontal_path_is_obstructed(m.origin, delta_x, state),
        (_, 0) => !horizontal_path_is_obstructed(m.origin, delta_x, state),
        (x, y) => x == y && !diagonal_is_obstructed(m.origin, m.origin, state),
    }

}

fn king_move_is_legal(m: &Move, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.origin, m.destination);

    return match (delta_x.abs(), delta_y.abs()) {
        (0, 1) => !horizontal_path_is_obstructed(m.origin, delta_x, state),
        (1, 0) => !horizontal_path_is_obstructed(m.origin, delta_x, state),
        (1, 1) => !diagonal_is_obstructed(m.origin, m.origin, state),
        _ => false,
    }
}

// Determine the horizontal distance between two squares
pub fn position_delta(origin: usize, destination: usize) -> (i32, i32) {
    let x = (destination as i32 % 8) - (origin as i32 % 8);
    let y = (destination as i32 / 8) - (origin as i32 / 8);
    return (x, y);
}

