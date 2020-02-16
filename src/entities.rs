
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
            let piece = Piece::new(placement.color, placement.piece);
            board.place_piece(piece, placement.square);
        }
        board
    }
    pub fn new() -> GameState {
        GameState::with_placements(vec![
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
        ])
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
pub struct Move {
    pub piece: PieceName,
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
    pub name: PieceName,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, name: PieceName) -> Piece {
        Piece {
            color,
            name,
            has_moved: false,
        }
    }
}


