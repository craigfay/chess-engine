use crate::pieces::{
    Piece,
    PieceName,
    Color,
};

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

impl ToString for GameState {
    // Create a human readable gamestate string
    fn to_string(&self) -> String {
        let mut output = String::with_capacity(136);
    
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = 8 * rank + file;
                match self.squares[square] {
                    None => output.push('.'),
                    Some(piece) => {
                        match (piece.color, piece.name) {
                            (Color::White, PieceName::Pawn) => output.push('P'),
                            (Color::White, PieceName::Bishop) => output.push('B'),
                            (Color::White, PieceName::Knight) => output.push('N'),
                            (Color::White, PieceName::Rook) => output.push('R'),
                            (Color::White, PieceName::Queen) => output.push('Q'),
                            (Color::White, PieceName::King) => output.push('K'),
                            (Color::Black, PieceName::Pawn) => output.push('p'),
                            (Color::Black, PieceName::Bishop) => output.push('b'),
                            (Color::Black, PieceName::Knight) => output.push('n'),
                            (Color::Black, PieceName::Rook) => output.push('r'),
                            (Color::Black, PieceName::Queen) => output.push('q'),
                            (Color::Black, PieceName::King) => output.push('k'),
                        }
                    }
                }
                output.push_str(" ");
            }
            output.push_str("\n");
        }
        output 
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

