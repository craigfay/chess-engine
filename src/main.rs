
#[derive(Copy)]
#[derive(Clone)]
struct GameBoard {
    squares: [Option<PieceType>; 64]
}

impl GameBoard {
    fn new() -> GameBoard {
        GameBoard {
            squares: [None; 64]
        }
    }
    fn place_piece(&mut self, piece: PieceType, square: usize) -> bool {
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

enum ActionType {
    Move,
    Capture,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
enum PieceType {
    Pawn,
}

struct Move {
    action: ActionType,
    piece: PieceType,
    destination: usize,
}

struct PawnRules {}

impl PawnRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        true
    }
}



fn main() {
    let mut board = GameBoard::new();
    let pawn: PieceType = PieceType::Pawn;
    board.place_piece(pawn, 0);
}
