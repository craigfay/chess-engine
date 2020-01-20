
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
enum Color {
    Black,
    White,
}


#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
struct Piece {
    color: Color,
    name: PieceType,
    has_moved: bool,
}

impl Piece {
    fn new(color: Color, name: PieceType) -> Piece {
        Piece {
            color,
            name,
            has_moved: false,
        }
    }
}

struct GameRules {}

impl GameRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let maybe_piece = board.squares[chosen_move.origin];

        // If there is no piece present at the chosen origin
        if maybe_piece.is_none() {
            return false
        }

        match chosen_move.piece {
            PieceType:: Pawn => PawnRules::can_move(chosen_move, board),
            _ => false
        }
    }
}

struct PawnRules {}

impl PawnRules {
    fn can_move(chosen_move: Move, board: GameBoard) -> bool {
        let piece = board.squares[chosen_move.origin].unwrap();

        return match piece.color {

            Color::White => {
                if chosen_move.destination == chosen_move.origin + 16 {
                    return true
                }
                else if false == piece.has_moved && chosen_move.destination == chosen_move.origin + 16 {
                    return true
                }
                return false
            },

            Color::Black => false,
        }
    }
}

fn delta_x(origin: usize, destination: usize) -> i32 {
    (destination as i32 % 8) - (origin as i32 % 8)
}

fn delta_y(origin: usize, destination: usize) -> i32 {
    (destination as i32 / 8) - (origin as i32 / 8)
}

fn delta_x_test() {
    assert_eq!(delta_x(0, 1), 1);
    assert_eq!(delta_x(0, 4), 4);
    assert_eq!(delta_x(0, 12), 4);
    assert_eq!(delta_x(12, 0), -4);
}

fn delta_y_test() {
    assert_eq!(delta_y(0, 1), 0);
    assert_eq!(delta_y(0, 56), 7);
    assert_eq!(delta_y(0, 12), 1);
    assert_eq!(delta_y(63, 0), -7);
}

fn main() {
    let mut board = GameBoard::new();
    let pawn: Piece = Piece::new(Color::White, PieceType::Pawn);
    board.place_piece(pawn, 0);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: PieceType::Pawn,
        origin: 0,
        destination: 8,
    };

    let result = GameRules::can_move(chosen_move, board);

    println!("{}", result);

    delta_x_test();
    delta_y_test();

}

