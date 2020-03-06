use crate::entities::{
    GameState,
    Color::{White,Black},
    PieceName::{
        Pawn,
        Bishop,
        Knight,
        Rook,
        Queen,
        King,
    }
};

// Create a human readable gamestate string
pub fn print(state: &GameState) -> String {
    let mut output = String::from("");

    for rank in (0..8).rev() {
        for file in 0..8 {
            let square = 8 * rank + file;
            match state.squares[square] {
                None => output.push_str("."),
                Some(piece) => {
                    match (piece.color, piece.name) {
                        (White, Pawn) => output.push_str("P"),
                        (White, Bishop) => output.push_str("B"),
                        (White, Knight) => output.push_str("N"),
                        (White, Rook) => output.push_str("R"),
                        (White, Queen) => output.push_str("Q"),
                        (White, King) => output.push_str("K"),
                        (Black, Pawn) => output.push_str("p"),
                        (Black, Bishop) => output.push_str("b"),
                        (Black, Knight) => output.push_str("n"),
                        (Black, Rook) => output.push_str("r"),
                        (Black, Queen) => output.push_str("q"),
                        (Black, King) => output.push_str("k"),
                    }
                }
            }
            output.push_str(" ");
        }
        output.push_str("\n");
    }
    output 
}

