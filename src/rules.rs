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
    Capture,
    Castle,
    CastleDirection::{Kingside, Queenside},
    Promotion,
    EnPassant,
    Color,
    Color::{White, Black},
};

fn piece_char(maybe_piece: Option<Piece>) -> String {
    match maybe_piece {
        None => String::from(""),
        Some(piece) => {
            match piece.name {
                Pawn => String::from(""),
                Bishop => String::from("B"),
                Knight => String::from("N"),
                Rook => String::from("R"),
                Queen => String::from("Q"),
                King => String::from("K"),
            }
        }
    }
}

use std::cmp::{min, max};

impl Action for Move {
    fn name(&self) -> &str {
        "Move"
    }
    fn as_algebraic_notation(&self, state: &GameState) -> String {
        if !self.is_legal(&state) {
            return String::from("");
        }
        let piece = piece_char(state.squares[self.from]);

        let origin_rank = (self.from  as u8 % 8 + 97) as char;
        let origin_file = (self.from / 8) + 1;

        let destination_rank = (self.to as u8 % 8 + 97) as char;
        let destination_file = (self.to / 8) + 1;

        String::from(format!(
            "{}{}{}",
            piece,
            destination_rank,
            destination_file
        ))
    }
    fn is_legal(&self, state: &GameState) -> bool {
        // Don't allow moves onto another piece
        if state.squares[self.to].is_some() {
            return false
        }
        // If there is no piece present at the chosen from
        if state.squares[self.from].is_none() {
            return false
        }
        // Don't allow moves with the same from/to
        if self.from == self.to {
            return false
        }
        // Don't allow moves to/from nonexistent squares
        if self.from > 63 || self.to > 63 {
            return false
        }
        // Verify that the pieces are allowed to move in accordance
        // with the specified to/from squares
        if !move_is_pseudo_legal(self.from, self.to, &state) {
            return false
        }
        // Don't allow moves that leave the current player checked
        if color_is_checked(state.to_move, &self.apply(&state)) {
            return false
        }
        true
    }

    fn apply(&self, state: &GameState) -> GameState {
        let mut new_state = state.clone();
        let (delta_x, delta_y) = position_delta(self.from, self.to);
    
        // En-Passant opportunities must expire after each turn
        new_state.en_passant_square = None;

        // Switch turns
        match new_state.to_move {
            White => new_state.to_move = Black,
            Black => new_state.to_move = White,
        }
    
        let piece = state.squares[self.from].unwrap();

        // Handle two square pawn advances
        if piece.name == Pawn {
            if delta_y.abs() == 2 {
                match state.squares[self.from] {
                    None => (),
                    Some(pawn) => {
                        match pawn.color {
                            White => new_state.en_passant_square = Some(self.from + 8),
                            Black => new_state.en_passant_square = Some(self.from - 8),
                        }
                    },
                }
            }
        }
    
        new_state.squares[self.to] = new_state.squares[self.from];
        new_state.squares[self.from] = None;
        new_state
    }
}

impl Action for Capture {
    fn name(&self) -> &str {
        "Capture"
    }
    fn as_algebraic_notation(&self, state: &GameState) -> String {
        return String::from("");
    }
    fn is_legal(&self, state: &GameState) -> bool {
        // If there is no piece present at the chosen origin 
        if state.squares[self.with].is_none() {
            return false
        }
        // If there is no piece present at the chosen destination
        if state.squares[self.on].is_none() {
            return false
        }
        // Don't allow moves with the same origin/destination
        if self.with == self.on {
            return false
        }
        // Don't allow moves to/from nonexistent squares
        if self.with> 63 || self.on > 63 {
            return false
        }

        let attacker = state.squares[self.with].unwrap();
        let defender = state.squares[self.on].unwrap();

        // Don't allow captures with the opponent's pieces
        if attacker.color != state.to_move {
            return false
        }
        // Don't allow players to capture their own pieces
        if defender.color == state.to_move {
            return false
        }

        // Verify that the pieces are allowed to move in accordance
        // with the specified to/from squares
        if !move_is_pseudo_legal(self.with, self.on, &state) {
            return false
        }
        // Don't allow moves that leave the current player checked
        if color_is_checked(state.to_move, &self.apply(&state)) {
            return false
        }
        true
    }

    fn apply(&self, state: &GameState) -> GameState {
        let mut new_state = state.clone();
        let (delta_x, delta_y) = position_delta(self.with, self.on);
    
        // En-Passant opportunities must expire after each turn
        new_state.en_passant_square = None;

        // Switch turns
        match new_state.to_move {
            White => new_state.to_move = Black,
            Black => new_state.to_move = White,
        }
        new_state.squares[self.on] = new_state.squares[self.with];
        new_state.squares[self.with] = None;
        new_state
    }
}

impl Action for EnPassant {
    fn name(&self) -> &str {
        "EnPassant"
    }
    fn as_algebraic_notation(&self, state: &GameState) -> String {
        return String::from("");
    }
    fn is_legal(&self, state: &GameState) -> bool {
        // Make sure en-passant is available
        if !state.en_passant_square.is_some() {
            return false
        }

        // Make sure there a piece on the attacking square
        if !state.squares[self.with].is_some() {
            return false
        }

        // Make sure there a pawn on the attacking square
        if (state.squares[self.with].unwrap()).name != Pawn {
            return false
        }

        let destination = state.en_passant_square.unwrap();
        let delta = position_delta(self.with, destination);

        let is_pseudo_legal = match state.squares[self.with] {
            None => false,
            Some(piece) => {
                // Check that the piece being moved is a pawn, that the vertical
                // movement corresponds to the pawn's color, and that the pawn's
                // color matches the current player's color.
                match (piece.color, delta) {
                    (White, ( 1,  1)) => state.to_move == White,
                    (White, (-1,  1)) => state.to_move == White,
                    (Black, ( 1, -1)) => state.to_move == Black,
                    (Black, (-1, -1)) => state.to_move == Black,
                    _ => false,
                }
            }
        };

        if !is_pseudo_legal {
            return false
        }

        // Don't allow actions that put/leave the player in check
        if color_is_checked(state.to_move, &self.apply(&state)) {
            return false
        }

        true
    }
    fn apply(&self, state: &GameState) -> GameState {
        let mut new_state = state.clone();
    
        let attacker = state.squares[self.with].unwrap();
        let destination = state.en_passant_square.unwrap();

        // Remove pawn that made en-passant eligable
        match attacker.color {
            White => new_state.squares[destination - 8] = None,
            Black => new_state.squares[destination + 8] = None,
        }
        
        // Expire the en-passant opportunity
        new_state.en_passant_square = None;

        // Switch turns
        match new_state.to_move {
            White => new_state.to_move = Black,
            Black => new_state.to_move = White,
        }

        // Move the attacking pawn into its new location
        let destination = state.en_passant_square.unwrap();
        new_state.squares[destination] = new_state.squares[self.with];
        new_state.squares[self.with] = None;
        new_state
    }
}

impl Action for Castle {
    fn name(&self) -> &str {
        "Castle"
    }
    fn as_algebraic_notation(&self, state: &GameState) -> String {
        return String::from("");
    }
    fn is_legal(&self, state: &GameState) -> bool {
        // Don't allow actions that put/leave the player in check
        if color_is_checked(state.to_move, &self.apply(&state)) {
            return false
        }
        match (state.to_move, &self.direction) {
            (White, Kingside) => {
                 if !state.white_can_castle_kingside { return false }
                 if !piece_is(White, King, 4, &state) { return false }
                 if !piece_is(White, Rook, 7, &state) { return false }
                 if state.squares[5].is_some() { return false }
                 if state.squares[6].is_some() { return false }
                 if color_threatens_square(Black, 4, &state) { return false }
                 if color_threatens_square(Black, 5, &state) { return false }
                 if color_threatens_square(Black, 6, &state) { return false }
                 if color_threatens_square(Black, 7, &state) { return false }
                 return true
            }
            (White, Queenside) => {
                if !state.white_can_castle_queenside { return false }
                if !piece_is(White, King, 4, &state) { return false }
                if !piece_is(White, Rook, 0, &state) { return false }
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
            (Black, Kingside) => {
                if !state.black_can_castle_kingside { return false }
                if !piece_is(Black, King, 60, &state) { return false }
                if !piece_is(Black, Rook, 63, &state) { return false }
                if state.squares[61].is_some() { return false }
                if state.squares[62].is_some() { return false }
                if color_threatens_square(White, 60, &state) { return false }
                if color_threatens_square(White, 61, &state) { return false }
                if color_threatens_square(White, 62, &state) { return false }
                if color_threatens_square(White, 63, &state) { return false }
                return true
            },
            (Black, Queenside) => {
                if !state.black_can_castle_queenside { return false }
                if !piece_is(Black, King, 60, &state) { return false }
                if !piece_is(Black, Rook, 56, &state) { return false }
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
    fn apply(&self, state: &GameState) -> GameState {
        let mut new_state = state.clone();

        // En-Passant opportunities must expire after each turn
        new_state.en_passant_square = None;

        // Switch turns
        match new_state.to_move {
            White => new_state.to_move = Black,
            Black => new_state.to_move = White,
        }

        match (state.to_move, &self.direction) {
            (White, Kingside) => {
                new_state.white_can_castle_kingside = false;
                new_state.squares[6] = new_state.squares[4];
                new_state.squares[5] = new_state.squares[7];
                new_state.squares[4] = None;
                new_state.squares[7] = None;
            },
            (White, Queenside) => {
                new_state.white_can_castle_queenside = false;
                new_state.squares[2] = new_state.squares[4];
                new_state.squares[3] = new_state.squares[0];
                new_state.squares[4] = None;
                new_state.squares[0] = None;
            },
            (Black, Kingside) => {
                new_state.black_can_castle_kingside = false;
                new_state.squares[62] = new_state.squares[60];
                new_state.squares[61] = new_state.squares[63];
                new_state.squares[60] = None;
                new_state.squares[63] = None;
            },
            (Black, Queenside) => {
                new_state.black_can_castle_queenside = false;
                new_state.squares[58] = new_state.squares[60];
                new_state.squares[59] = new_state.squares[56];
                new_state.squares[60] = None;
                new_state.squares[56] = None;
            }
        }
        new_state
    }
}

impl Action for Promotion {
    fn name(&self) -> &str {
        "Promotion"
    }
    fn as_algebraic_notation(&self, state: &GameState) -> String {
        return String::from("");
    }
    fn is_legal(&self, state: &GameState) -> bool {
        if !pawn_can_promote_to(&self.pawn_becomes) {
            return false
        }

        // Don't allow actions that put/leave the player in check
        if color_is_checked(state.to_move, &self.apply(&state)) {
            return false
        }

        match state.squares[self.moving_from] {
            None => false,
            Some(piece) => {
                if piece.color != state.to_move {
                    return false
                }
                match piece.name {
                    Pawn => {
                        match piece.color {
                            White => {
                                if self.moving_from < 47 { return false }
                                if self.moving_from > 56 { return false }
                                if state.squares[self.moving_from + 8].is_some() { return false }
                                true
                            },
                            Black => {
                                if self.moving_from < 8 { return false }
                                if self.moving_from > 15 { return false }
                                if state.squares[self.moving_from - 8].is_some() { return false }
                                true
                            },
                        }
                    }
                    _ => false
                }
            }
        }
    }
    fn apply(&self, state: &GameState) -> GameState {
        let mut new_state = state.clone();

        // En-Passant opportunities must expire after each turn
        new_state.en_passant_square = None;

        // Switch turns
        match new_state.to_move {
            White => new_state.to_move = Black,
            Black => new_state.to_move = White,
        }

        // Remove pawn
        new_state.squares[self.moving_from] = None;

        // Place new piece
        new_state.squares[self.to] = Some(Piece {
            name: self.pawn_becomes,
            color: state.to_move,
        });

        new_state
    }
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
    for direction in [Kingside, Queenside].iter() {
        let action = Castle { direction: *direction };
        if action.is_legal(&state) {
            results.push(action);
        }
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

pub fn pawn_can_promote_to(piece: &PieceName) -> bool {
    match piece {
        Bishop => true,
        Knight => true,
        Rook => true,
        Queen => true,
        _ => false,
    }
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

// Determine the horizontal distance between two squares
pub fn position_delta(from: usize, to: usize) -> (i32, i32) {
    let x = (to as i32 % 8) - (from as i32 % 8);
    let y = (to as i32 / 8) - (from as i32 / 8);
    return (x, y);
}

fn count_pieces_that_can_move_to_square(piece_name: PieceName, square: usize, state: &GameState) -> i32 {
    let mut count = 0;

    for origin in 0..64 {
        if state.squares[origin].is_some() {
            let piece = state.squares[origin].unwrap();
            if piece.color == state.to_move && piece.name == piece_name {
                let action = Move { from: origin, to: square };
                if action.is_legal(&state) {
                    count += 1;
                }
            }

        }
    }
    count
}

