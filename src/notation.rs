// The single responsibility of this module is to translate between real-world
// chess notation conventions, and the application's representation of equivalent concepts.

use crate::entities::{
    Move,
    GameState,
    PieceName::{
        Pawn
    },
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
