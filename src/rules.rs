// The single responsibility of this module is to determine what type of actions
// are legal to perform on an arbitrary GameState.

use crate::entities::{
    GameState,
    PieceName,
    PieceName::{
        Pawn,
        Bishop,
        Knight,
        Rook,
        Queen,
        King,
    },
    Piece,
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

pub fn color_threatens_square(color: Color, target_square: usize, state: &GameState) -> bool {
    for (square, maybe_piece) in state.squares.iter().enumerate() {
        match maybe_piece {
            Some(piece) => {
                if piece.color != color {
                    continue;
                }
                let m = Move {
                    origin: square,
                    destination: target_square,
                    piece: piece.name,
                };

                // Not all pawn moves are threatening
                if piece.name == Pawn {
                    let (delta_x, delta_y) = position_delta(m.origin, m.destination);
                    if (delta_x.abs(),  delta_y.abs()) != (1, 1) {
                        continue;
                    }
                }

                if move_is_pseudo_legal(&m, &state) {
                    return true
                }
            },
            None => continue,
        }
    }
    false
}

pub fn color_is_checked(color: Color, state: &GameState) -> bool {
    let mut king_square: Option<usize> = None;
     
    // Determine which square the king of active color is on
    for (square, maybe_piece) in state.squares.iter().enumerate() {
        match maybe_piece {
            Some(piece) => {
                if piece.name != King { continue; }
                if piece.color != color { continue; }
                king_square = Some(square);
                break;
            },
            None => continue,
        }
    }

    if !king_square.is_some() {
        return false
    }

    // Determine whether the other color is threatening king_square
    let attacker = if state.to_move == White { Black } else { White };
    color_threatens_square(attacker, king_square.unwrap(), &state)
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

    // It may also be problematic that move_is_legal and color_is_checked
    // call one another.
    
    if is_legal_castle(&m, &state) { return true }

    move_is_pseudo_legal(&m, &state) 
}

// Determine whether the pieces can move in accordance
// with a given move, regardless of threats or non-placement
// game state
fn move_is_pseudo_legal(m: &Move, state: &GameState) -> bool {
     return match m.piece {
        Pawn => pawn_move_is_legal(m, state),
        Rook => rook_move_is_legal(m, state),
        Bishop => bishop_move_is_legal(m, state),
        Knight => knight_move_is_legal(m, state),
        Queen => queen_move_is_legal(m, state),
        King => king_move_is_legal(m, state),
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

fn is_legal_castle(m: &Move, state: &GameState) -> bool {
    if state.to_move == White && m.origin == 4 && m.destination == 7 {
        if !state.white_can_castle_kingside { return false }
        // Rook must be on the correct square
        if !piece_is(White, Rook, state.squares[7]) { return false }
        // In-between squares must be empty
        if state.squares[5].is_some() { return false }
        if state.squares[6].is_some() { return false }
        // Must not be, or travel through/into/out of check
        if color_threatens_square(Black, 4, &state) { return false }
        if color_threatens_square(Black, 5, &state) { return false }
        if color_threatens_square(Black, 6, &state) { return false }
        if color_threatens_square(Black, 7, &state) { return false }
        return true
    }
    if state.to_move == Black && m.origin == 60 && m.destination == 63 {
        if !state.black_can_castle_kingside { return false }
        if !piece_is(Black, Rook, state.squares[63]) { return false }
        if state.squares[61].is_some() { return false }
        if state.squares[62].is_some() { return false }
        if color_threatens_square(White, 60, &state) { return false }
        if color_threatens_square(White, 61, &state) { return false }
        if color_threatens_square(White, 62, &state) { return false }
        if color_threatens_square(White, 63, &state) { return false }
        return true
    }
    if state.to_move == White && m.origin == 4 && m.destination == 0 {
        if !state.white_can_castle_queenside { return false }
        if !piece_is(White, Rook, state.squares[0]) { return false }
        if state.squares[1].is_some() { return false }
        if state.squares[2].is_some() { return false }
        if state.squares[3].is_some() { return false }
        if color_threatens_square(Black, 0, &state) { return false }
        if color_threatens_square(Black, 1, &state) { return false }
        if color_threatens_square(Black, 2, &state) { return false }
        if color_threatens_square(Black, 3, &state) { return false }
        if color_threatens_square(Black, 4, &state) { return false }
        return true
    }
    if state.to_move == Black && m.origin == 60 && m.destination == 56 {
        if !state.black_can_castle_queenside { return false }
        if !piece_is(Black, Rook, state.squares[56]) { return false }
        if state.squares[57].is_some() { return false }
        if state.squares[58].is_some() { return false }
        if state.squares[59].is_some() { return false }
        if color_threatens_square(White, 56, &state) { return false }
        if color_threatens_square(White, 57, &state) { return false }
        if color_threatens_square(White, 58, &state) { return false }
        if color_threatens_square(White, 59, &state) { return false }
        if color_threatens_square(White, 60, &state) { return false }
        return true
    }
    false
}

pub fn piece_is(color: Color, name: PieceName, actual: Option<Piece>) -> bool {
    match actual {
        Some(piece) => piece.color == color && piece.name == name,
        None => false
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

