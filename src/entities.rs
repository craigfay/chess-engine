// The single responsibility of this module is to define data structures
// that represent real world chess objects and game states.

#[derive(Copy)]
#[derive(Clone)]
pub struct GameState {
    pub squares: [Option<Piece>; 64],
    pub to_move: Color,
    pub black_can_castle_kingside: bool,
    pub white_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
    pub white_can_castle_queenside: bool,
    pub en_passant_square: Option<usize>,
}

impl GameState {
    pub fn empty() -> GameState {
        GameState {
            squares: [None; 64],
            to_move: Color::White,
            black_can_castle_kingside: false,
            white_can_castle_kingside: false,
            black_can_castle_queenside: false,
            white_can_castle_queenside: false,
            en_passant_square: None,
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
            black_can_castle_kingside: false,
            white_can_castle_kingside: false,
            black_can_castle_queenside: false,
            white_can_castle_queenside: false,
            en_passant_square: None,
        };
        for placement in placements.iter() {
            let piece = Piece::new(placement.color, placement.piece);
            board.place_piece(piece, placement.square);
        }
        board
    }
    pub fn new() -> GameState {
        let mut state = GameState::with_placements(vec![
            Placement::new(Color::White, PieceName::Rook, 0),
            Placement::new(Color::White, PieceName::Knight, 1),
            Placement::new(Color::White, PieceName::Bishop, 2),
            Placement::new(Color::White, PieceName::Queen, 3),
            Placement::new(Color::White, PieceName::King, 4),
            Placement::new(Color::White, PieceName::Bishop, 5),
            Placement::new(Color::White, PieceName::Knight, 6),
            Placement::new(Color::White, PieceName::Rook, 7),
    
            Placement::new(Color::White, PieceName::Pawn, 8),
            Placement::new(Color::White, PieceName::Pawn, 9),
            Placement::new(Color::White, PieceName::Pawn, 10),
            Placement::new(Color::White, PieceName::Pawn, 11),
            Placement::new(Color::White, PieceName::Pawn, 12),
            Placement::new(Color::White, PieceName::Pawn, 13),
            Placement::new(Color::White, PieceName::Pawn, 14),
            Placement::new(Color::White, PieceName::Pawn, 15),
    
            Placement::new(Color::Black, PieceName::Pawn, 48),
            Placement::new(Color::Black, PieceName::Pawn, 49),
            Placement::new(Color::Black, PieceName::Pawn, 50),
            Placement::new(Color::Black, PieceName::Pawn, 51),
            Placement::new(Color::Black, PieceName::Pawn, 52),
            Placement::new(Color::Black, PieceName::Pawn, 53),
            Placement::new(Color::Black, PieceName::Pawn, 54),
            Placement::new(Color::Black, PieceName::Pawn, 55),
    
            Placement::new(Color::Black, PieceName::Rook, 56),
            Placement::new(Color::Black, PieceName::Knight, 57),
            Placement::new(Color::Black, PieceName::Bishop, 58),
            Placement::new(Color::Black, PieceName::Queen, 59),
            Placement::new(Color::Black, PieceName::King, 60),
            Placement::new(Color::Black, PieceName::Bishop, 61),
            Placement::new(Color::Black, PieceName::Knight, 62),
            Placement::new(Color::Black, PieceName::Rook, 63),
        ]);

        // Set castling state
        state.black_can_castle_kingside = true;
        state.white_can_castle_kingside = true;
        state.black_can_castle_queenside = true;
        state.white_can_castle_queenside = true;
        state.en_passant_square = None;

        state
    }
}

#[derive(Debug)]
pub struct Placement {
    pub color: Color,
    pub piece: PieceName,
    pub square: usize,
}

impl Placement {
    pub  fn new(color: Color, piece: PieceName, square: usize) -> Placement {
        Placement { color, piece, square }
    }
}

impl std::fmt::Debug for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();

        for i in 0..self.squares.len() {
            let square_str = format!("\n{} {:?}", i, self.squares[i]);
            output.push_str(&square_str);
        }

        write!(f, "{}", output)
    }
}

pub trait Action {
    fn is_legal(&self, state: &GameState) -> bool;
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum PieceName {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub enum CastleDirection {
    Kingside,
    Queenside,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub struct Castle {
    pub direction: CastleDirection,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Promotion {
    pub pawn_becomes: PieceName,
    pub moving_from: usize,
    pub to: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Move {
    pub piece: PieceName,
    pub from: usize,
    pub to: usize,
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
    pub name: PieceName,
}

impl Piece {
    pub fn new(color: Color, name: PieceName) -> Piece {
        Piece { color, name }
    }
}




