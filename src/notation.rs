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

// Create a move from algebraic notation
pub fn algebraic_move(s: &str, state: GameState) -> Option<Move> {
    
    let dummy_move = Move { piece: Pawn, from: 0 as usize,
        to: 1 as usize,
    };

    // static PIECES: [char; 5] = ['B','N','R','Q','K'];

    let chars: Vec<char> = s.chars().collect();

    match chars.as_slice() {
        // Pawn Moves (a6, b2, h8, etc..)
        [file, rank] => {
            // TODO: Reexamine this to make sure we're not doing more logic than is necessary
            let square = algebraic(s);
            if false == square.is_some() { return None }
            let square = square.unwrap() as usize;
            if state.to_move == White {
                if square < 16 { return None }
                if state.squares[square - 8].is_some() {
                    return Some(Move {
                        from: square - 8,
                        to: square,
                        piece: Pawn,
                    })
                }
                else {
                    return  Some(Move {
                        from: square - 16,
                        to: square,
                        piece: Pawn,
                    })
                }
            }
            else {
                if square >= 48 { return None }
                if state.squares[square + 8].is_some() {
                    return Some(Move {
                        from: square + 8,
                        to: square,
                        piece: Pawn,
                    })
                }
                else {
                    return  Some(Move {
                        from: square + 16,
                        to: square,
                        piece: Pawn,
                    })
                }
            }
        },
        _ => None,
    }
}



// Get algebraic notation from index
pub fn index(square: usize) -> String {
    let rank = (square as u8 % 8 + 97) as char;
    let file = (square / 8) + 1;
    String::from(format!("{}{}", rank, file))
}

// Get index notation from algebraic
pub fn algebraic(s: &str) -> Option<usize> {
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

