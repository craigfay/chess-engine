#[derive(Copy, Clone)]
struct Pawn;

type Piece = Pawn;

struct GameBoard {
    squares: [Option<Piece>; 64]
}

impl GameBoard {
    fn new() -> GameBoard {
        GameBoard {
            squares: [None; 64]
        }
    }
}

fn main() {
    let board = GameBoard::new();
}
