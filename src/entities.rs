
#[derive(Copy)]
#[derive(Clone)]
pub struct GameBoard {
    pub squares: [Option<Piece>; 64]
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            squares: [None; 64]
        }
    }
    pub fn place_piece(&mut self, piece: Piece, square: usize) -> bool {
        if square > 63 {
            return false;
        }
        if (self.squares[square]).is_some() {
            // Maybe panic here instead
            return false;
        }
        self.squares[square] = Some(piece);
        return true;
    }
    pub fn with_placements(placements: Vec<Placement>) -> GameBoard {
        let mut board = GameBoard {
            squares: [None; 64]
        };
        for placement in placements.iter() {
            let piece = Piece::new(placement.color, placement.piece_type);
            board.place_piece(piece, placement.square);
        }
        board
    }
}

pub struct Placement {
    piece_type: PieceType,
    color: Color,
    square: usize,
}

impl Placement {
    pub  fn new(piece_type: PieceType, color: Color, square: usize) -> Placement {
        Placement { piece_type, color, square }
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

pub enum ActionType {
    Move,
    Capture,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Rook,
    Bishop,
    Knight,
}

pub struct Move {
    pub action: ActionType,
    pub piece: PieceType,
    pub origin: usize,
    pub destination: usize,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Piece {
    pub color: Color,
    pub name: PieceType,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, name: PieceType) -> Piece {
        Piece {
            color,
            name,
            has_moved: false,
        }
    }
}


