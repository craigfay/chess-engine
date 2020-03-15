// The single responsibility of this module is to define types of
// actions that can be applied to a GameState

use crate::gamestate::GameState;

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

use crate::utilities::{
    color_threatens_square,
    color_is_checked,
    legal_actions,
    legal_next_states,
    position_delta,
    movement_is_vertical,
    move_is_pseudo_legal,
    piece_is,
};


pub trait Action {
    fn is_legal(&self, state: &GameState) -> bool;
    fn apply(&self, state: &GameState) -> GameState;
    fn name(&self) -> &str;
    fn as_algebraic_notation(&self, state: &GameState) -> String;
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum CastleDirection {
    Kingside,
    Queenside,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub struct Castle {
    pub direction: CastleDirection,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Promotion {
    pub pawn_becomes: PieceName,
    pub moving_from: usize,
    pub to: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Capture {
    pub on: usize,
    pub with: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct EnPassant {
    pub with: usize,
}

impl Action for Move {
    fn name(&self) -> &str {
        "Move"
    }
    fn as_algebraic_notation(&self, state: &GameState) -> String {
        if !self.is_legal(&state) {
            return String::from("");
        }
        let piece_str = state.squares[self.from].unwrap().to_string();

        let mut origin_rank = &mut String::with_capacity(1);
        let mut origin_file = &mut String::with_capacity(1);
        let destination_file = (self.to as u8 % 8 + 97) as char;
        let destination_rank = (self.to / 8) + 1;

        let ambiguity = disambiguate_move(self.from, self.to, &state);

        if ambiguity.rank_is_ambiguous {
            origin_rank.push((self.from as u8 / 8 + 1 + 48 ) as char);
        }
        if ambiguity.file_is_ambiguous {
            origin_file.push((self.from as u8 % 8 + 97) as char);
        }

        String::from(format!(
            "{}{}{}{}{}",
            piece_str,
            origin_file,
            origin_rank,
            destination_file,
            destination_rank,
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
        if !self.is_legal(&state) {
            return String::from("");
        }
        let piece_str = state.squares[self.with].unwrap().to_string();

        let mut origin_rank = &mut String::with_capacity(1);
        let mut origin_file = &mut String::with_capacity(1);
        let destination_file = (self.on as u8 % 8 + 97) as char;
        let destination_rank = (self.on / 8) + 1;

        let ambiguity = disambiguate_capture(self.with, self.on, &state);

        if ambiguity.rank_is_ambiguous {
            origin_rank.push((self.with as u8 / 8 + 1 + 48 ) as char);
        }
        if piece_str == "" || ambiguity.file_is_ambiguous {
            origin_file.push((self.with as u8 % 8 + 97) as char);
        }

        String::from(format!(
            "{}{}{}x{}{}",
            piece_str,
            origin_file,
            origin_rank,
            destination_file,
            destination_rank,
        ))
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

        // Don't allow pawns to attack vertically. Vertical movement in
        // the correct direction is pseudo-legal for pawns, so it's an
        // edge case that the following block prevents.
        if attacker.name == Pawn && movement_is_vertical(self.with, self.on) {
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
        if !self.is_legal(&state) {
            return String::from("");
        }

        let mut origin_file = &mut String::with_capacity(1);
        origin_file.push((self.with as u8 % 8 + 97) as char);

        let destination_file = (state.en_passant_square.unwrap() as u8 % 8 + 97) as char;
        let destination_rank = (state.en_passant_square.unwrap() as u8 / 8) + 1;

        String::from(format!(
            "{}x{}{}",
            origin_file,
            destination_file,
            destination_rank,
        ))

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
        if !self.is_legal(&state) {
            return String::from("");
        }
        match self.direction {
            CastleDirection::Kingside => String::from("O-O"),
            CastleDirection::Queenside => String::from("O-O-O"),
        }
    }
    fn is_legal(&self, state: &GameState) -> bool {
        // Don't allow actions that put/leave the player in check
        if color_is_checked(state.to_move, &self.apply(&state)) {
            return false
        }
        match (state.to_move, &self.direction) {
            (White, CastleDirection::Kingside) => {
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
            (White, CastleDirection::Queenside) => {
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
            (Black, CastleDirection::Kingside) => {
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
            (Black, CastleDirection::Queenside) => {
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
            (White, CastleDirection::Kingside) => {
                new_state.white_can_castle_kingside = false;
                new_state.squares[6] = new_state.squares[4];
                new_state.squares[5] = new_state.squares[7];
                new_state.squares[4] = None;
                new_state.squares[7] = None;
            },
            (White, CastleDirection::Queenside) => {
                new_state.white_can_castle_queenside = false;
                new_state.squares[2] = new_state.squares[4];
                new_state.squares[3] = new_state.squares[0];
                new_state.squares[4] = None;
                new_state.squares[0] = None;
            },
            (Black, CastleDirection::Kingside) => {
                new_state.black_can_castle_kingside = false;
                new_state.squares[62] = new_state.squares[60];
                new_state.squares[61] = new_state.squares[63];
                new_state.squares[60] = None;
                new_state.squares[63] = None;
            },
            (Black, CastleDirection::Queenside) => {
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
        if !self.is_legal(&state) {
            return String::from("");
        }

        let new_piece = Piece {
            color: state.to_move,
            name: self.pawn_becomes,
        };

        let new_piece_str = new_piece.to_string();

        let mut origin_file = &mut String::with_capacity(1);
        let mut capture_indicator = &mut String::with_capacity(1);

        // Only use origin_file / capture_indicator if the promotion
        // captures an enemy piece
        if !movement_is_vertical(self.moving_from, self.to) {
            origin_file.push((self.moving_from as u8 % 8 + 97) as char);
            capture_indicator.push('x');
        }

        let destination_file = (self.to as u8 % 8 + 97) as char;
        let destination_rank = (self.to / 8) + 1;

        String::from(format!(
            "{}{}{}{}{}",
            origin_file,
            capture_indicator,
            destination_file,
            destination_rank,
            new_piece_str,
        ))
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

pub fn pawn_can_promote_to(piece: &PieceName) -> bool {
    match piece {
        Bishop => true,
        Knight => true,
        Rook => true,
        Queen => true,
        _ => false,
    }
}

struct Disambiguation {
    rank_is_ambiguous: bool,
    file_is_ambiguous: bool,
}

fn disambiguate_move(origin: usize, destination: usize, state: &GameState) -> Disambiguation {
    let mut rank_is_ambiguous = false;
    let mut file_is_ambiguous = false;

    let piece_name = state.squares[origin].unwrap().name;

    for square in 0..64 {
        if square == origin {
            continue;
        }
        if state.squares[square].is_some() {
            let piece = state.squares[square].unwrap();
            if piece.name == piece_name && piece.color == state.to_move {
                let action = Move { from: square, to: destination };
                if action.is_legal(&state) {
                    if origin as u8 % 8 == square as u8 % 8 {
                        rank_is_ambiguous = true;
                    }
                    if origin / 8 == square / 8 {
                        file_is_ambiguous = true;
                    }
                }
            }

        }
    }
    Disambiguation { rank_is_ambiguous, file_is_ambiguous }
}

fn disambiguate_capture(origin: usize, destination: usize, state: &GameState) -> Disambiguation {
    let mut rank_is_ambiguous = false;
    let mut file_is_ambiguous = false;

    let piece_name = state.squares[origin].unwrap().name;

    for square in 0..64 {
        if square == origin {
            continue;
        }
        if state.squares[square].is_some() {
            let piece = state.squares[square].unwrap();
            if piece.name == piece_name && piece.color == state.to_move {
                let action = Capture { on: destination, with: square } ;
                if action.is_legal(&state) {
                    if origin as u8 % 8 == square as u8 % 8 {
                        rank_is_ambiguous = true;
                    }
                    if origin / 8 == square / 8 {
                        file_is_ambiguous = true;
                    }
                }
            }

        }
    }
    Disambiguation { rank_is_ambiguous, file_is_ambiguous }
}

