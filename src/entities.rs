
#[derive(Copy)]
#[derive(Clone)]
pub struct GameState {
    pub squares: [Option<Piece>; 64],
    pub to_move: Color,
}

impl GameState {
    pub fn empty() -> GameState {
        GameState {
            squares: [None; 64],
            to_move: Color::White,
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
    pub fn with_placements(placements: Vec<Placement>) -> GameState {
        let mut board = GameState {
            squares: [None; 64],
            to_move: Color::White,
        };
        for placement in placements.iter() {
            let piece = Piece::new(placement.color, placement.piece_type);
            board.place_piece(piece, placement.square);
        }
        board
    }
    pub fn new() -> GameState {
        GameState::with_placements(vec![
            Placement::new(Color::White, PieceNames::Rook, 0),
            Placement::new(Color::White, PieceNames::Knight, 1),
            Placement::new(Color::White, PieceNames::Bishop, 2),
            Placement::new(Color::White, PieceNames::Queen, 3),
            Placement::new(Color::White, PieceNames::King, 4),
            Placement::new(Color::White, PieceNames::Bishop, 5),
            Placement::new(Color::White, PieceNames::Knight, 6),
            Placement::new(Color::White, PieceNames::Rook, 7),
    
            Placement::new(Color::White, PieceNames::Pawn, 8),
            Placement::new(Color::White, PieceNames::Pawn, 9),
            Placement::new(Color::White, PieceNames::Pawn, 10),
            Placement::new(Color::White, PieceNames::Pawn, 11),
            Placement::new(Color::White, PieceNames::Pawn, 12),
            Placement::new(Color::White, PieceNames::Pawn, 13),
            Placement::new(Color::White, PieceNames::Pawn, 14),
            Placement::new(Color::White, PieceNames::Pawn, 15),
    
            Placement::new(Color::Black, PieceNames::Pawn, 48),
            Placement::new(Color::Black, PieceNames::Pawn, 49),
            Placement::new(Color::Black, PieceNames::Pawn, 50),
            Placement::new(Color::Black, PieceNames::Pawn, 51),
            Placement::new(Color::Black, PieceNames::Pawn, 52),
            Placement::new(Color::Black, PieceNames::Pawn, 53),
            Placement::new(Color::Black, PieceNames::Pawn, 54),
            Placement::new(Color::Black, PieceNames::Pawn, 55),
    
            Placement::new(Color::Black, PieceNames::Rook, 56),
            Placement::new(Color::Black, PieceNames::Knight, 57),
            Placement::new(Color::Black, PieceNames::Bishop, 58),
            Placement::new(Color::Black, PieceNames::Queen, 59),
            Placement::new(Color::Black, PieceNames::King, 60),
            Placement::new(Color::Black, PieceNames::Bishop, 61),
            Placement::new(Color::Black, PieceNames::Knight, 62),
            Placement::new(Color::Black, PieceNames::Rook, 63),
        ])
    }
}

pub struct Placement {
    color: Color,
    piece_type: PieceNames,
    square: usize,
}

impl Placement {
    pub  fn new(color: Color, piece_type: PieceNames, square: usize) -> Placement {
        Placement { color, piece_type, square }
    }
}
    

impl std::fmt::Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();

        for i in 0..self.squares.len() {
            let square_str = format!("\n{:?}", self.squares[i]);
            output.push_str(&square_str);
        }

        write!(f, "{}", output)
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PieceNames {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Move {
    pub piece: PieceNames,
    pub origin: usize,
    pub destination: usize,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub struct Piece {
    pub color: Color,
    pub name: PieceNames,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, name: PieceNames) -> Piece {
        Piece {
            color,
            name,
            has_moved: false,
        }
    }
}


