struct Piece;

struct GameBoard {
    squares: Vec<Option<Piece>>
}

fn main() {
    let board = GameBoard {
        squares: vec![
            Some(Piece {}),
            None,
        ]
    };
}
