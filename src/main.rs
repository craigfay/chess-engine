
#[derive(Copy)]
#[derive(Clone)]
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
    origin: usize,
    destination: usize,
}


#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Piece {
    name: PieceType,
    has_moved: bool,
}

impl Piece {
    fn new(name: PieceType) -> Piece {
        Piece {
            name,
            has_moved: false,
        }
    }
}

struct GameRules {}

impl GameRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let square = board.squares[chosen_move.origin];
        let square_has_piece = match square {
            Some(square) => true,
            None => false,
        };
        square_has_piece
    }
}

struct PawnRules {}

impl PawnRules {

}



fn main() {
    let mut board = GameBoard::new();
    let pawn: Piece = Piece::new(PieceType::Pawn);
    board.place_piece(pawn, 0);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: PieceType::Pawn,
        origin: 1,
        destination: 8,
    };

    let result = GameRules::can_move(chosen_move, board);

    println!("{}", result);
}

