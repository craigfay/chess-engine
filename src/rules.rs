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
    Action,
    Move,
    Castle,
    Promotion,
    Color,
    CastleDirection::{Kingside, Queenside},
    Color::{White, Black},
};

impl Action for Move {
    fn is_legal(&self, state: &GameState) -> bool {
       move_is_legal(&self, &state)
    }
}

impl Action for Castle {
    fn is_legal(&self, state: &GameState) -> bool {
        match state.to_move {
            White => {
                match &self.direction {
                    Kingside => {
                        if !state.white_can_castle_kingside { return false }
                        if !piece_is(White, Rook, state.squares[7]) { return false }
                        if state.squares[5].is_some() { return false }
                        if state.squares[6].is_some() { return false }
                        if color_threatens_square(Black, 4, &state) { return false }
                        if color_threatens_square(Black, 5, &state) { return false }
                        if color_threatens_square(Black, 6, &state) { return false }
                        if color_threatens_square(Black, 7, &state) { return false }
                        return true
                    },
                    Queenside => {
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
                }
            },
            Black => {
                match &self.direction {
                    Kingside => {
                        if !state.black_can_castle_kingside { return false }
                        if !piece_is(Black, Rook, state.squares[63]) { return false }
                        if state.squares[61].is_some() { return false }
                        if state.squares[62].is_some() { return false }
                        if color_threatens_square(White, 60, &state) { return false }
                        if color_threatens_square(White, 61, &state) { return false }
                        if color_threatens_square(White, 62, &state) { return false }
                        if color_threatens_square(White, 63, &state) { return false }
                        return true
                    },
                    Queenside => {
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
                }
            }
        }
    }
}

use crate::notation::{algebraic};

use std::cmp::{min, max};

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

pub fn state_after_move(m: &Move, state: &GameState) -> GameState {
    let mut new_state = state.clone();
    let (delta_x, delta_y) = position_delta(m.from, m.to);

    // En-Passant opportunities must expire after each turn
    new_state.en_passant_square = None;

    // Handle two square pawn advances
    if m.piece == Pawn {
        if delta_y.abs() == 2 {
            match state.squares[m.from] {
                None => (),
                Some(pawn) => {
                    match pawn.color {
                        White => new_state.en_passant_square = Some(m.from + 8),
                        Black => new_state.en_passant_square = Some(m.from - 8),
                    }
                },
            }
        }
    }

    // Handle en-passant captures
    if m.piece == Pawn && Some(m.to) == state.en_passant_square {
        match state.squares[m.from] {
            None => (),
            Some(pawn) => {
                match pawn.color {
                    White => new_state.squares[m.to - 8] = None,
                    Black => new_state.squares[m.to + 8] = None,
                }
            }

        }
    }

    // Handle castling
    if castle_is_legal(&m, &state) {
        match (m.from, m.to) {
            (4, 6) => {
                new_state.white_can_castle_kingside = false;
                new_state.squares[6] = new_state.squares[4];
                new_state.squares[5] = new_state.squares[7];
                new_state.squares[4] = None;
                new_state.squares[7] = None;
            },
            (4, 2) => {
                new_state.white_can_castle_queenside = false;
                new_state.squares[2] = new_state.squares[4];
                new_state.squares[3] = new_state.squares[0];
                new_state.squares[4] = None;
                new_state.squares[0] = None;
            },
            (60, 62) => {
                new_state.black_can_castle_kingside = false;
                new_state.squares[62] = new_state.squares[60];
                new_state.squares[61] = new_state.squares[63];
                new_state.squares[60] = None;
                new_state.squares[63] = None;
            },
            (60, 58) => {
                new_state.black_can_castle_queenside = false;
                new_state.squares[58] = new_state.squares[60];
                new_state.squares[59] = new_state.squares[56];
                new_state.squares[60] = None;
                new_state.squares[56] = None;
            },
            _ => panic!("Castle is considered legal but not playable"),
        }
    }
    else {
        // Non-Castling moves
        new_state.squares[m.to] = new_state.squares[m.from];
        new_state.squares[m.from] = None;
    }

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
                    from: square,
                    to: target_square,
                    piece: piece.name,
                };

                // Not all pawn moves are threatening
                if piece.name == Pawn {
                    let (delta_x, delta_y) = position_delta(m.from, m.to);
                    if (delta_x.abs(),  delta_y.abs()) == (1, 1) { return true; }
                    else { continue; }
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

    for from in 0..64 {
        if state.squares[from].is_some() {
            let piece = state.squares[from].unwrap();
            if piece.color == state.to_move {
                for to in 0..64 {
                    let m = Move { from, to, piece: piece.name };;
                    if move_is_legal(&m, state) {
                        results.push(m);
                    }
                }
            }

        }
    }
    results
}

pub fn promotion_is_legal(p: &Promotion, state: &GameState) -> bool {
    if false == pawn_can_promote_to(&p.pawn_becomes) { return false }

    match state.squares[p.moving_from] {
        None => false,
        Some(piece) => {
            if piece.color != state.to_move {
                return false
            }
            match piece.name {
                Pawn => {
                    match piece.color {
                        White => {
                            if p.moving_from < 47 { return false }
                            if p.moving_from > 56 { return false }
                            if state.squares[p.moving_from + 8].is_some() { return false }
                            true
                        },
                        Black => {
                            if p.moving_from < 8 { return false }
                            if p.moving_from > 15 { return false }
                            if state.squares[p.moving_from - 8].is_some() { return false }
                            true
                        },
                    }
                }
                _ => false
            }
        }
    }
}

pub fn pawn_can_promote_to(piece: &PieceName) -> bool {
    match piece {
        Bishop => true,
        Knight => true,
        Rook => true,
        Queen => true,
        _ => false,
    }
}

pub fn move_is_legal(m: &Move, state: &GameState) -> bool {
    // Don't allow moves with the same from/to
    if m.from == m.to { return false }

    // Don't allow moves to/from nonexistent squares
    if m.from > 63 || m.to > 63 { return false }

    let maybe_piece = state.squares[m.from];
    let to = state.squares[m.to];

    // If there is no piece present at the chosen from
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
    
    if castle_is_legal(&m, &state) { return true }

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
    let piece = state.squares[m.from].unwrap();

    let (delta_x, delta_y)  = position_delta(m.from, m.to);

    let to_is_enemy_piece = match state.squares[m.to] {
        Some(other_piece) => other_piece.color != piece.color,
        None => Some(m.to) == state.en_passant_square,
    };

    match piece.color {
        // White Pieces can only move upwards
        White => {
            match (delta_x, delta_y) {
                // Normal Moves
                (0, 1) => true,
                // Two-Square Moves
                (0, 2) => m.from > 7 && m.from < 16,
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
                (0, -2) => m.from > 47 && m.from < 56,
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
    for x in 1..delta_y.abs() {
        let index = if delta_y > 0 {
            from + x as usize * 8
        } else {
            from - x as usize * 8
        };
        if state.squares[index].is_some() {
            return true
        }
    }
    false
}

fn diagonal_is_obstructed(from: usize, to: usize, state: &GameState) -> bool {
    let low = min(from, to);
    let hi = max(from, to);

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
    let (delta_x, delta_y)  = position_delta(m.from, m.to);
    // Return false if the path is obstructed
    if horizontal_path_is_obstructed(m.from, delta_x, state) {
        return false;
    }

    match (delta_x, delta_y) {
        (0, _) => true,
        (_, 0) => true,
        _ => false,
    }
}

fn bishop_move_is_legal(m: &Move, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.from, m.to);
    if delta_x.abs() != delta_y.abs() {
        return false;
    }
    return false == diagonal_is_obstructed(m.from, m.from, state);
}


fn knight_move_is_legal(m: &Move, _state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.from, m.to);

    return match (delta_x.abs(), delta_y.abs()) {
        (1, 2) => true,
        (2, 1) => true,
        (_, _) => false,
    }

}

fn queen_move_is_legal(m: &Move, state: &GameState) -> bool {
    let (delta_x, delta_y)  = position_delta(m.from, m.to);

    return match (delta_x.abs(), delta_y.abs()) {
        (0, _) => !horizontal_path_is_obstructed(m.from, delta_x, state),
        (_, 0) => !horizontal_path_is_obstructed(m.from, delta_x, state),
        (x, y) => x == y && !diagonal_is_obstructed(m.from, m.from, state),
    }

}

fn castle_is_legal(m: &Move, state: &GameState) -> bool {
    if state.to_move == White && m.from == 4 && m.to == 6 {
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
    if state.to_move == Black && m.from == 60 && m.to == 62 {
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
    if state.to_move == White && m.from == 4 && m.to == 2 {
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
    if state.to_move == Black && m.from == 60 && m.to == 58 {
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
    let (delta_x, delta_y)  = position_delta(m.from, m.to);
    return match (delta_x.abs(), delta_y.abs()) {
        (0, 1) => !horizontal_path_is_obstructed(m.from, delta_x, state),
        (1, 0) => !horizontal_path_is_obstructed(m.from, delta_x, state),
        (1, 1) => !diagonal_is_obstructed(m.from, m.from, state),
        _ => false,
    }
}

// Determine the horizontal distance between two squares
pub fn position_delta(from: usize, to: usize) -> (i32, i32) {
    let x = (to as i32 % 8) - (from as i32 % 8);
    let y = (to as i32 / 8) - (from as i32 / 8);
    return (x, y);
}

