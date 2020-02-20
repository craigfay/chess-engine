// The single responsibility of this module is to determine what type of actions
// are legal to perform on an arbitrary GameState.

use crate::entities::{
    GameState,
    PieceName,
    Move,
    Color,
    Color::{White, Black},
};

use crate::notation::{algebraic};

use std::cmp::{min, max};

pub fn state_after_move(m: &Move, state: &GameState) -> GameState {
    let mut new_state = state.clone();
    new_state.squares[m.destination] = new_state.squares[m.origin];
    new_state.squares[m.origin] = None;
    new_state
}

pub fn square_is_threatened(target_square: usize, state: &GameState) -> bool {
    for (square, maybe_piece) in state.squares.iter().enumerate() {
        match maybe_piece {
            Some(piece) => {
                if piece.color != state.to_move { continue; }
                let m = Move {
                    origin: square,
                    destination: target_square,
                    piece: piece.name,
                };
                if move_is_legal(&m, &state) { return true }
            },
            None => continue,
        }
    }
    false
}

pub fn color_is_checked(color: Color, state: &GameState) -> bool {

    // index 64 does not represent a board square.
    // We're using it to indicate that the king square is unknown.
    // An Option type would probably be better.
    let mut king_square: usize = 64;
     
    // Determine which square the king of active color is on
    for (square, maybe_piece) in state.squares.iter().enumerate() {
        match maybe_piece {
            Some(piece) => {
                if piece.name != PieceName::King { continue; }
                if piece.color != color { continue; }
                king_square = square;
                break;
            },
            None => continue,
        }
    }

    if king_square == 64 {
        // There is no king on the board, so it can't be check
        return false
    }

    // Determine whether the inactive color is threatening king_square
    for (square, maybe_piece) in state.squares.iter().enumerate() {
        match maybe_piece {
            Some(piece) => {
                if piece.color == state.to_move { continue; }
                let m = Move {
                    origin: square,
                    destination: king_square,
                    piece: piece.name,
                };
                if move_is_legal(&m, &state) { return true }
            },
            None => continue,
        }
    }
    false
}

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
    // Don't allow moves with the same origin/destination
    if m.origin == m.destination { return false }

    // Don't allow moves to/from nonexistent squares
    if m.origin > 63 || m.destination > 63 { return false }

    let maybe_piece = state.squares[m.origin];
    let destination = state.squares[m.destination];

    // If there is no piece present at the chosen origin
    if maybe_piece.is_none() {
        return false
    }

    // Don't allow moves that leave the current player checked
    // The function may be more efficient if this block gets moved
    // below individual piece rules, because of how often it runs
    // with illegal moves by legal_moves().
    let aftermath = state_after_move(&m, &state);
    if color_is_checked(state.to_move, &aftermath) {
        return false;
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

