// The single responsibility of this module is to translate between real-world
// chess notation conventions, and the application's representation of equivalent concepts.

use crate::gamestate::GameState;

use crate::pieces::{
    PieceName,
    Color::{White,Black},
};

// Get algebraic notation from index
pub fn square_index_to_algebraic(square: usize) -> String {
    let rank = (square as u8 % 8 + 97) as char;
    let file = (square / 8) + 1;
    String::from(format!("{}{}", rank, file))
}

// Get index notation from algebraic
pub fn square_algebraic_to_index(s: &str) -> Option<usize> {
    static RANKS: [char; 8] = ['1','2','3','4','5','6','7','8'];
    static FILES: [char; 8] = ['a','b','c','d','e','f','g','h'];

    let chars: Vec<char> = s.chars().collect();

    match chars.as_slice() {
        [file, rank] => {
            if FILES.contains(file) && RANKS.contains(rank) {
                let file = *file as usize - 97;
                let rank = (rank.to_digit(10).unwrap() as usize - 1) * 8;
                return Some(file + rank)
            }
            None
        },
        _ => None
    }
}

// Create a human readable gamestate string
pub fn fen_notation(state: &GameState) -> String {
    let mut output = String::new();

    // Piece Placement
    for rank in (0..8).rev() {
        let mut empty_spaces: u8 = 0;
        for file in 0..8 {
            let square = 8 * rank + file;

            // End of rank
            if (square + 1) % 8 == 0  && square != 63 {
                output.push('/');
                empty_spaces = 0;
            }

            match state.squares[square] {
                None => empty_spaces += 1,
                Some(piece) => {
                    if empty_spaces > 0 {
                        output.push(empty_spaces as char);
                    }
                    match (piece.color, piece.name) {
                        (White, PieceName::Pawn) => output.push('P'),
                        (White, PieceName::Bishop) => output.push('B'),
                        (White, PieceName::Knight) => output.push('N'),
                        (White, PieceName::Rook) => output.push('R'),
                        (White, PieceName::Queen) => output.push('Q'),
                        (White, PieceName::King) => output.push('K'),
                        (Black, PieceName::Pawn) => output.push('p'),
                        (Black, PieceName::Bishop) => output.push('b'),
                        (Black, PieceName::Knight) => output.push('n'),
                        (Black, PieceName::Rook) => output.push('r'),
                        (Black, PieceName::Queen) => output.push('q'),
                        (Black, PieceName::King) => output.push('k'),
                    }
                }
            }
        }
    }


    // Active Color
    output.push_str(" ");
    match state.to_move {
        White => output.push('w'),
        Black => output.push('b'),
    }

    // Castling Rights
    output.push_str(" ");
    if state.white_can_castle_kingside { output.push('K'); }
    if state.white_can_castle_queenside { output.push('Q'); }
    if state.black_can_castle_kingside { output.push('k'); }
    if state.black_can_castle_queenside { output.push('q'); }

    // En Passant
    output.push_str(" ");
    match state.en_passant_square {
        None => output.push('-'),
        Some(square) => output.push_str(&square_index_to_algebraic(square)),
    }

    // Halfmove clock
    // Note: This library isn't concerned with implementing 
    // clock state currently, so this value is meaningless.
    output.push_str(" ");
    output.push('0');

    // Fullmove clock
    // Note: This library isn't concerned with implementing 
    // clock state currently, so this value is meaningless.
    output.push_str(" ");
    output.push('0');

    output
}


