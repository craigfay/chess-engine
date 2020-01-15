
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Pawn;

impl Moveable for Pawn {
    fn can_move(origin: usize, destination: usize) -> bool {
        true
    }
}

trait Moveable {
    fn can_move(origin: usize, destination: usize) -> bool;
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
enum Piece {
    Pawn(Pawn)
}

struct GameBoard {
    squares: [Option<Piece>; 64]
}

impl GameBoard {
    fn new() -> GameBoard {
        GameBoard {
            squares: [None; 64]
        }
    }
    fn place_piece(&mut self, piece: Piece, square: usize) -> bool {
        if square < 0 || square > 63 {
            return false;
        }
        self.squares[square] = Some(piece);
        return true;
    }
}

impl std::fmt::Debug for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();

        for i in 0..self.squares.len() {
            let square_str = format!("\n{:?}", self.squares[i]);
            output.push_str(&square_str);
        }

        write!(f, "{}", output)
    }
}



fn main() {
    let mut board = GameBoard::new();
    let pawn: Piece = Piece::Pawn(Pawn{});
    board.place_piece(pawn, 0);
}
