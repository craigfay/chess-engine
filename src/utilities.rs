use crate::pieces::{
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
    Color,
    Color::{White, Black},
};

use crate::gamestate::GameState;

use crate::actions::{
    Action,
    Move,
    Capture,
    Promotion,
    Castle,
    CastleDirection::{Kingside, Queenside},
    EnPassant,
};

use std::cmp::{min, max};

pub fn color_threatens_square(color: Color, target_square: usize, state: &GameState) -> bool {
    for (square, maybe_piece) in state.squares.iter().enumerate() {
        match maybe_piece {
            Some(piece) => {
                if piece.color != color {
                    continue;
                }
                let m = Move { from: square, to: target_square };

                // Not all pawn moves are threatening
                if piece.name == Pawn {
                    let (delta_x, delta_y) = position_delta(m.from, m.to);
                    if (delta_x.abs(),  delta_y.abs()) == (1, 1) { return true; }
                    else { continue; }
                }

                if move_is_pseudo_legal(m.from, m.to, &state) {
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
    let attacker = if color == White { Black } else { White };
    color_threatens_square(attacker, king_square.unwrap(), &state)
}

pub fn legal_next_states(state: &GameState) -> Vec<GameState> {
    let mut results: Vec<GameState> = vec![];

    for action  in legal_moves(&state) {
        results.push(action.apply(&state));
    }
    for action  in legal_captures(&state) {
        results.push(action.apply(&state));
    }
    for action  in legal_en_passants(&state) {
        results.push(action.apply(&state));
    }
    for action in legal_castles(&state) {
        results.push(action.apply(&state));
    }
    for action in legal_promotions(&state) {
        results.push(action.apply(&state));
    }
    results
}


pub fn legal_actions(state: &GameState) -> Vec<Box<dyn Action>> {
    let mut results: Vec<Box<dyn Action>> = vec![];

    for action  in legal_moves(&state) {
        results.push(Box::new(action));
    }
    for action in legal_captures(&state) {
        results.push(Box::new(action));
    }
    for action  in legal_en_passants(&state) {
        results.push(Box::new(action));
    }
    for action in legal_castles(&state) {
        results.push(Box::new(action));
    }
    for action in legal_promotions(&state) {
        results.push(Box::new(action));
    }
    results
}

// Determine the horizontal distance between two squares
pub fn position_delta(from: usize, to: usize) -> (i32, i32) {
    let x = (to as i32 % 8) - (from as i32 % 8);
    let y = (to as i32 / 8) - (from as i32 / 8);
    return (x, y);
}

pub fn movement_is_vertical(origin: usize, destination: usize) -> bool {
    (origin as i32 - destination as i32).abs() % 8 == 0
}

pub fn relative_material_values(state: &GameState) -> (usize, usize) {
    let mut white = 0;
    let mut black = 0;

    for maybe_piece in state.squares.iter() {
        match maybe_piece {
            None => (),
            Some(piece) => {
                match piece.color {
                    White => white += piece_value(&piece.name),
                    Black => black += piece_value(&piece.name),
                }
            }
        }
    }

    (white, black)
}

pub fn piece_value(name: &PieceName) -> usize {
    match name {
        Pawn => 1,
        Bishop => 3,
        Knight => 3,
        Rook => 5,
        Queen => 9,
        King => 0,
    }
}

pub fn legal_en_passants(state: &GameState) -> Vec<EnPassant> {
    let mut results = vec![];
    if !state.en_passant_square.is_some() { return results }

    let destination = state.en_passant_square.unwrap();

    let action = EnPassant { with: destination - 7 };
    if action.is_legal(&state) { results.push(action); }

    let action = EnPassant { with: destination + 7 };
    if action.is_legal(&state) { results.push(action); }

    let action = EnPassant { with: destination - 9 };
    if action.is_legal(&state) { results.push(action); }

    let action = EnPassant { with: destination + 9 };
    if action.is_legal(&state) { results.push(action); }

    results
}

pub fn legal_moves(state: &GameState) -> Vec<Move> {
    let mut results = vec![];

    for from in 0..64 {
        if state.squares[from].is_some() {
            let piece = state.squares[from].unwrap();
            if piece.color == state.to_move {
                for to in 0..64 {
                    let action = Move { from, to };
                    if action.is_legal(&state) {
                        results.push(action);
                    }
                }
            }

        }
    }
    results
}

pub fn legal_captures(state: &GameState) -> Vec<Capture> {
    let mut results = vec![];

    for with in 0..64 {
        if state.squares[with].is_some() {
            let piece = state.squares[with].unwrap();
            if piece.color == state.to_move {
                for on in 0..64 {
                    let action = Capture { on, with };
                    if action.is_legal(&state) {
                        results.push(action);
                    }
                }
            }

        }
    }
    results
}

pub fn legal_castles(state: &GameState) -> Vec<Castle> {
    let mut results = vec![];
    let action = Castle { direction: Kingside };
    if action.is_legal(&state) {
        results.push(action);
    }
    let action = Castle { direction: Queenside };
    if action.is_legal(&state) {
        results.push(action);
    }
    results
}

pub fn legal_promotions(state: &GameState) -> Vec<Promotion> {
    let mut results = vec![];
    let promotion_targets: [PieceName; 4] = [Bishop, Knight, Rook, Queen];

    if state.to_move == White {
        for square in 48..56 {
            for target in promotion_targets.iter() {
                let action = Promotion {
                    pawn_becomes: *target,
                    moving_from: square,
                    to: square + 8
                };
                if action.is_legal(&state) {
                    results.push(action);
                }
            }
        }
    }
    else if state.to_move == Black {
        for square in 8..16 {
            for target in promotion_targets.iter() {
                let action = Promotion {
                    pawn_becomes: *target,
                    moving_from: square,
                    to: square - 8
                };
                if action.is_legal(&state) {
                    results.push(action);
                }
            }
        }
    }
    results
}

// Determine whether the pieces can move in accordance
// with a given move, regardless of threats or non-placement
// game state
fn move_is_pseudo_legal(origin: usize, destination: usize, state: &GameState) -> bool {
    if !state.squares[origin].is_some() {
        return false
    }

    let piece = state.squares[origin].unwrap();
    match piece.name {
        Pawn => pawn_move_is_legal(origin, destination, state),
        Rook => rook_move_is_legal(origin, destination, state),
        Bishop => bishop_move_is_legal(origin, destination, state),
        Knight => knight_move_is_legal(origin, destination, state),
        Queen => queen_move_is_legal(origin, destination, state),
        King => king_move_is_legal(origin, destination, state),
    }   
}

fn pawn_move_is_legal(origin: usize, destination: usize, state: &GameState) -> bool {
    let piece = state.squares[origin].unwrap();

    let (delta_x, delta_y)  = position_delta(origin, destination);

    let to_is_enemy_piece = match state.squares[destination] {
        Some(other_piece) => other_piece.color != piece.color,
        None => Some(destination) == state.en_passant_square,
    };

    match piece.color {
        // White Pieces can only move upwards
        White => {
            match (delta_x, delta_y) {
                // Normal Moves
                (0, 1) => true,
                // Two-Square Moves
                (0, 2) => origin > 7 && origin < 16,
                // Captures
                (1, 1) => to_is_enemy_piece,
                (-1, 1) => to_is_enemy_piece,
                _ => false,
            }
        },
        // Black pieces can only move downwards
        Black => {
            match (delta_x, delta_y) {
                // Normal Moves
                (0, -1) => true,
                // Two-Square Moves
                (0, -2) => origin > 47 && origin < 56,
                // Captures
                (1, -1) => to_is_enemy_piece,
                (-1, -1) => to_is_enemy_piece,
                _ => false,
            }
        }

    }
}

fn horizontal_path_is_obstructed(from: usize, delta_x: i32, state: &GameState) -> bool {
    for x in 1..delta_x.abs() {
        let index = if delta_x > 0 { from + x as usize } else { from - x as usize };
        if state.squares[index].is_some() {
            return true
        }
    }
    false
}

fn vertical_path_is_obstructed(from: usize, delta_y: i32, state: &GameState) -> bool {
    for y in 1..delta_y.abs() {
        let index = if delta_y > 0 {
            from + y as usize * 8
        } else {
            from - y as usize * 8
        };
        if state.squares[index].is_some() {
            return true
        }
    }
    false
}

fn diagonal_path_is_obstructed(from: usize, to: usize, state: &GameState) -> bool {
    let low = min(from, to);
    let hi = max(from, to);

    // The difference between two diagonal squares will divide by 7 or 9
    for n in [7,9].iter() {
        if (hi - low) % n == 0 {
            for i in (low+1..hi).step_by(*n) {
                if state.squares[i].is_some() {
                    return true
                } 
            }
        }
    }
    return false;
}

fn rook_move_is_legal(origin: usize, destination: usize, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(origin, destination);
    // Return false if the path is obstructed
    if horizontal_path_is_obstructed(origin, delta_x, state) {
        return false;
    }

    match (delta_x, delta_y) {
        (0, _) => true,
        (_, 0) => true,
        _ => false,
    }
}

fn bishop_move_is_legal(origin: usize, destination: usize, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(origin, destination);
    if delta_x.abs() != delta_y.abs() {
        return false;
    }
    return false == diagonal_path_is_obstructed(origin, destination, state);
}


fn knight_move_is_legal(origin: usize, destination: usize, _state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(origin, destination);

    return match (delta_x.abs(), delta_y.abs()) {
        (1, 2) => true,
        (2, 1) => true,
        (_, _) => false,
    }

}

fn queen_move_is_legal(origin: usize, destination: usize, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(origin, destination);

    return match (delta_x.abs(), delta_y.abs()) {
        (0, _) => !horizontal_path_is_obstructed(origin, delta_x, state),
        (_, 0) => !horizontal_path_is_obstructed(origin, delta_x, state),
        (x, y) => x == y && !diagonal_path_is_obstructed(origin, destination, state),
    }

}

pub fn piece_is(color: Color, name: PieceName, square: usize, state: &GameState) -> bool {
    match state.squares[square] {
        Some(piece) => piece.color == color && piece.name == name,
        None => false
    }
}

fn king_move_is_legal(origin: usize, destination: usize, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(origin, destination);
    return match (delta_x.abs(), delta_y.abs()) {
        (0, 1) => !horizontal_path_is_obstructed(origin, delta_x, &state),
        (1, 0) => !vertical_path_is_obstructed(origin, delta_y, &state),
        (1, 1) => !diagonal_path_is_obstructed(origin, destination, &state),
        _ => false,
    }
}


