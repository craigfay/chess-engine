
use crate::utilities::{
    color_threatens_square,
    color_is_checked,
    position_delta,
    legal_actions,
    is_checkmate,
    is_stalemate,
};

use crate::actions::{
    Action,
    Move,
    Capture,
    Promotion,
    Castle,
    CastleDirection::{Kingside, Queenside},
    EnPassant,
};

use crate::gamestate::{
    GameState,
    Placement,
};

use crate::pieces::{
    Piece,
    PieceName::{
        Pawn,
        Rook,
        Bishop,
        Knight,
        Queen,
        King,
    },
    Color::{White, Black},
};

use crate::notation::{
    square_index_to_algebraic,
    square_algebraic_to_index,
    fen_notation,
};

#[test]
fn new_gamestate_test() {
    // Gamestate can be constructed to represent a normal start
    let state = GameState::new();

    for squares in 8..16 {
        let piece = state.squares[squares].unwrap();
        assert!(piece.color == White && piece.name == Pawn);
    }

    let piece = state.squares[0].unwrap();
    assert!(piece.color == White && piece.name == Rook);
    let piece = state.squares[1].unwrap();
    assert!(piece.color == White && piece.name == Knight);
    let piece = state.squares[2].unwrap();
    assert!(piece.color == White && piece.name == Bishop);
    let piece = state.squares[3].unwrap();
    assert!(piece.color == White && piece.name == Queen);
    let piece = state.squares[4].unwrap();
    assert!(piece.color == White && piece.name == King);
    let piece = state.squares[5].unwrap();
    assert!(piece.color == White && piece.name == Bishop);
    let piece = state.squares[6].unwrap();
    assert!(piece.color == White && piece.name == Knight);
    let piece = state.squares[7].unwrap();
    assert!(piece.color == White && piece.name == Rook);

    for squares in 48..55 {
        let piece = state.squares[squares].unwrap();
        assert!(piece.color == Black && piece.name == Pawn);
    }

    let piece = state.squares[56].unwrap();
    assert!(piece.color == Black && piece.name == Rook);
    let piece = state.squares[57].unwrap();
    assert!(piece.color == Black && piece.name == Knight);
    let piece = state.squares[58].unwrap();
    assert!(piece.color == Black && piece.name == Bishop);
    let piece = state.squares[59].unwrap();
    assert!(piece.color == Black && piece.name == Queen);
    let piece = state.squares[60].unwrap();
    assert!(piece.color == Black && piece.name == King);
    let piece = state.squares[61].unwrap();
    assert!(piece.color == Black && piece.name == Bishop);
    let piece = state.squares[62].unwrap();
    assert!(piece.color == Black && piece.name == Knight);
    let piece = state.squares[63].unwrap();
    assert!(piece.color == Black && piece.name == Rook);

    for squares in 17..47 {
        assert!(state.squares[squares].is_none());
    }

    assert!(state.black_can_castle_kingside);
    assert!(state.white_can_castle_kingside);
    assert!(state.black_can_castle_queenside);
    assert!(state.white_can_castle_queenside);
}

#[test]
fn legal_actions_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, King, 60),
    ]);
    assert_eq!(5, legal_actions(&state).len());
}

#[test]
fn legal_actions_no_kings_test() {
    // There can be legal moves even if state lacks Kings
    //
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 8),
        Placement::new(White, Pawn, 9),
    ]);
    assert_eq!(4, legal_actions(&state).len());
}

#[test]
fn position_delta_test() {
    assert_eq!(position_delta(0, 1), (1, 0));
    assert_eq!(position_delta(0, 4), (4, 0));
    assert_eq!(position_delta(0, 12), (4, 1));
    assert_eq!(position_delta(12, 0), (-4, -1));

    assert_eq!(position_delta(0, 1), (1, 0));
    assert_eq!(position_delta(0, 56), (0, 7));
    assert_eq!(position_delta(0, 12), (4, 1));
    assert_eq!(position_delta(63, 0), (-7, -7));
}

#[test]
fn pawn_movement_sideways_test() {
    // Pawns should not be able to move sideways
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 16),
    ]);
    let action = Move { from: 16, to: 17 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn pawn_movement_too_far_test() {
    // Pawns should not be able to move more than two squares vertically
    let state = GameState::with_placements(vec![

        Placement::new(White, Pawn, 18),
    ]);
    let action = Move { from: 18, to: 42 };
    assert!(!action.is_legal(&state));
}

#[test]
fn pawn_movement_normal_test() {
    // White pawns should be able to move 1 square up
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 22),
    ]);
    let action = Move { from: 22, to: 30 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn pawn_cant_move_diagonally_test() {
    // Pawns should not be able to move diagonally
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 4),
    ]);
    let action = Move { from: 22, to: 31 };
    assert_eq!(false, action.is_legal(&state));

    let action = Move { from: 22, to: 29 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn rook_movement_horizontal_test() {
    // Rooks should be able to travel horizontally
    let state = GameState::with_placements(vec![
        Placement::new(White, Rook, 35),
    ]);
    let action = Move { from: 35, to: 32 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn rook_movement_vertical_test() {
    // Rooks should be able to travel vertically
    let state = GameState::with_placements(vec![
        Placement::new(White, Rook, 35),
    ]);
    let action = Move { from: 35, to: 3 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn rook_movement_horizontal_obstruction_test() {
    // Rooks should not be able to travel horizontally through other pieces
    let state = GameState::with_placements(vec![
        Placement::new(White, Rook, 32),
        Placement::new(Black, Pawn, 33),
    ]);
    let action = Move { from: 32, to: 36 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn rook_movement_vertical_obstruction_test() {
    // Rooks should not be able to travel vertically through other pieces
    let state = GameState::new();
    let action = Move { from: 0, to: 8 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn bishop_movement_diagonal_up_left_test() {
    // Bishops should be able to travel diagonally up-left
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 22),
    ]);
    let action = Move { from: 22, to: 36 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn bishop_movement_diagonal_up_right_test() {
    // Bishops should be able to travel diagonally up-right
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 0),
    ]);
    let action = Move { from: 0, to: 36 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn bishop_movement_diagonal_down_left_test() {
    // Bishops should be able to travel diagonally down-left
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 27),
    ]);
    let action = Move { from: 27, to: 9 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn bishop_movement_diagonal_down_right_test() {
    // Bishops should be able to travel diagonally down-right
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 56),
    ]);
    let action = Move { from: 56, to: 42 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn bishop_movement_diagonal_right_edge_test() {
    // Bishops shouldn't be able to wrap around the right edge of the state
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 23),
    ]);
    let action = Move { from: 23, to: 41 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn bishop_movement_diagonal_left_edge_test() {
    // Bishops shouldn't be able to wrap around the left edge of the state
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 24),
    ]);
    let action = Move { from: 24, to: 15 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn knight_movement_two_up_one_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 45 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn knight_movement_one_up_two_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 38 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn knight_movement_two_up_one_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 43 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn knight_movement_one_up_two_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 34 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn knight_movement_two_down_one_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 13 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn knight_movement_one_down_two_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 22 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn knight_movement_two_down_one_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 11 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn knight_movement_one_down_two_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { from: 28, to: 18 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn queen_movement_horizontal_test() {
    // Queens should be able to move horizontally
    let state = GameState::with_placements(vec![
        Placement::new(Black, Queen, 24),
    ]);
    let action = Move { from: 24, to: 30 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn queen_movement_vertical_obstruction_test() {
    // Queens should not be able to travel vertically through other pieces
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Queen, 40),
        Placement::new(Black, Pawn, 32),
    ]);
    state.to_move = Black;
    let action = Move { from: 40, to: 24 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn queen_movement_horizontal_obstruction_test() {
    // Queens should not be able to travel horizontally through other pieces
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Queen, 40),
        Placement::new(Black, Pawn, 41),
    ]);
    state.to_move = Black;
    let action = Move { from: 40, to: 42 };
    assert_eq!(false, action.is_legal(&state));
}

#[test]
fn queen_movement_vertical_test() {
    // Queens should be able to move horizontally
    let state = GameState::with_placements(vec![
        Placement::new(Black, Queen, 24),
    ]);
    let action = Move { from: 24, to: 48 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn queen_movement_diagonal_test() {
    // Queens should be able to move diagonally
    let state = GameState::with_placements(vec![
        Placement::new(White, Queen, 24),
    ]);
    let action = Move { from: 24, to: 42 };
    assert_eq!(true, action.is_legal(&state));
}

#[test]
fn king_movement_horizontal_test() {
    // Kings should be able to move one square horizontally
    let state = GameState::with_placements(vec![
        Placement::new(Black, King, 28),
    ]);
    let action = Move { from: 28, to: 27 };
    assert!(action.is_legal(&state));
}

#[test]
fn king_movement_vertical_test() {
    // Kings should be able to move one square vertically
    let state = GameState::with_placements(vec![
        Placement::new(Black, King, 28),
    ]);
    let action = Move { from: 28, to: 20 };
    assert!(action.is_legal(&state));
}

#[test]
fn king_movement_diagonal_test() {
    // Kings should be able to move one square diagonally
    let state = GameState::with_placements(vec![
        Placement::new(Black, King, 24),
    ]);
    let action = Move { from: 24, to: 33 };
    assert!(action.is_legal(&state));
}

#[test]
fn cant_move_onto_another_piece_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, King, 63),
        Placement::new(Black, Pawn, 52),
        Placement::new(White, Pawn, 44),
        Placement::new(White, Bishop, 43),
        Placement::new(White, Knight, 35),
        Placement::new(White, Rook, 53),
        Placement::new(White, Queen, 51),
        Placement::new(White, King, 60),
    ]);

    let action = Move { from: 44, to:52  };
    assert!(!action.is_legal(&state));

    let action = Move { from: 43, to:52  };
    assert!(!action.is_legal(&state));

    let action = Move { from: 35, to:52  };
    assert!(!action.is_legal(&state));

    let action = Move { from: 53, to:52  };
    assert!(!action.is_legal(&state));

    let action = Move { from: 51, to:52  };
    assert!(!action.is_legal(&state));
}

#[test]
fn algebraic_notation_to_index_test() {
    let ranks = ["1","2","3","4","5","6","7","8"];
    let files = ["a","b","c","d","e","f","g","h"];

    for (r, rank) in ranks.iter().enumerate() {
        for (f, file) in files.iter().enumerate() {
            let square = format!("{}{}", file, rank);
            let index = r * 8 + f;
            assert_eq!(square_algebraic_to_index(&square), Some(index));
        }
    }
}

#[test]
fn color_is_checked_test() {
    let state = GameState::new();
    assert!(!color_is_checked(White, &state));

    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 56),
        Placement::new(White, Rook, 0),
    ]);
    state.to_move = Black;
    assert!(color_is_checked(Black, &state));
}

#[test]
fn color_threatens_square_test() {
    let state = GameState::new();
    assert!(color_threatens_square(White, 20, &state));
    assert!(color_threatens_square(Black, 44, &state));

    // Forward pawn moves are not threatening
    assert!(!color_threatens_square(White, 28, &state));
    assert!(!color_threatens_square(Black, 36, &state));
}

#[test]
fn state_after_move_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 0),
        Placement::new(Black, Rook, 56),
    ]);   
    let action = Move { from: 0, to: 1 };
    let new_state = action.apply(&state);
    assert!(new_state.squares[1].unwrap().name == King);
    assert!(!new_state.squares[0].is_some());
}

#[test]
fn white_kingside_castle_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    assert!(action.is_legal(&state));
}
  
#[test]
fn black_kingside_castle_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
    ]);
    state.black_can_castle_kingside = true;
    state.to_move = Black;
    let action = Castle { direction: Kingside };
    assert!(action.is_legal(&state));
}

#[test]
fn white_queenside_castle_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    assert!(action.is_legal(&state));
}
  
#[test]
fn black_queenside_castle_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 56),
    ]);
    state.black_can_castle_queenside = true;
    state.to_move = Black;
    let action = Castle { direction: Queenside };
    assert!(action.is_legal(&state));
}

#[test]
fn white_kingside_castle_aftermath_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    let aftermath = action.apply(&state);
    let king = aftermath.squares[6].unwrap();
    let rook = aftermath.squares[5].unwrap();
    assert!(king.name == King && king.color == White);
    assert!(rook.name == Rook && rook.color == White);
    assert!(!aftermath.squares[4].is_some());
    assert!(!aftermath.squares[7].is_some());
}

#[test]
fn white_queenside_castle_aftermath_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    let aftermath = action.apply(&state);
    let king = aftermath.squares[2].unwrap();
    let rook = aftermath.squares[3].unwrap();
    assert!(king.name == King && king.color == White);
    assert!(rook.name == Rook && rook.color == White);
    assert!(!aftermath.squares[4].is_some());
    assert!(!aftermath.squares[0].is_some());
}

#[test]
fn black_kingside_castle_aftermath_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
    ]);
    state.black_can_castle_kingside = true;
    state.to_move = Black;
    let action = Castle { direction: Kingside };
    let aftermath = action.apply(&state);
    let king = aftermath.squares[62].unwrap();
    let rook = aftermath.squares[61].unwrap();
    assert!(king.name == King && king.color == Black);
    assert!(rook.name == Rook && rook.color == Black);
    assert!(!aftermath.squares[60].is_some());
    assert!(!aftermath.squares[63].is_some());
}

#[test]
fn black_queenside_castle_aftermath_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 56),
    ]);
    state.black_can_castle_queenside = true;
    state.to_move = Black;
    let action = Castle { direction: Queenside };
    let aftermath = action.apply(&state);
    let king = aftermath.squares[58].unwrap();
    let rook = aftermath.squares[59].unwrap();
    assert!(king.name == King && king.color == Black);
    assert!(rook.name == Rook && rook.color == Black);
    assert!(!aftermath.squares[60].is_some());
    assert!(!aftermath.squares[56].is_some());
}

#[test]
fn white_kingside_castle_obstruction_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Bishop, 5),
        Placement::new(White, Rook, 7),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_queenside_castle_obstruction_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Queen, 3),
        Placement::new(White, Rook, 0),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_kingside_castle_obstruction_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Knight, 62),
        Placement::new(Black, Rook, 63),
    ]);
    state.black_can_castle_kingside = true;
    state.to_move = Black;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_queenside_castle_obstruction_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Bishop, 58),
        Placement::new(Black, Rook, 56),
    ]);
    state.black_can_castle_queenside = true;
    state.to_move = Black;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_kingside_castle_out_of_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
        Placement::new(Black, Rook, 21),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_kingside_castle_into_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
        Placement::new(Black, Pawn, 15),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_kingside_castle_through_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
        Placement::new(Black, Bishop, 12),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_queenside_castle_out_of_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
        Placement::new(Black, Rook, 28),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_queenside_castle_into_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
        Placement::new(Black, Knight, 17),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_queenside_castle_through_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
        Placement::new(Black, Queen, 59),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_kingside_castle_out_of_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
        Placement::new(White, Pawn, 51),
    ]);
    state.black_can_castle_kingside = true;
    state.to_move = Black;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_kingside_castle_into_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
        Placement::new(White, Bishop, 55),
    ]);
    state.black_can_castle_kingside = true;
    state.to_move = Black;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_kingside_castle_through_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
        Placement::new(White, Bishop, 52),
    ]);
    state.black_can_castle_kingside = true;
    state.to_move = Black;
    let action = Castle { direction: Kingside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_queenside_castle_out_of_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 56),
        Placement::new(White, Pawn, 53),
    ]);
    state.black_can_castle_queenside = true;
    state.to_move = Black;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_queenside_castle_into_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 56),
        Placement::new(White, Pawn, 49),
    ]);
    state.black_can_castle_queenside = true;
    state.to_move = Black;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_queenside_castle_through_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 56),
        Placement::new(White, Rook, 51),
    ]);
    state.black_can_castle_queenside = true;
    state.to_move = Black;
    let action = Castle { direction: Queenside };
    assert!(!action.is_legal(&state));
}

#[test]
fn moving_black_queenside_rook_removes_black_queenside_castle_rights_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 56),
        Placement::new(White, Rook, 51),
    ]);
    state.black_can_castle_queenside = true;
    state.to_move = Black;

    let action = Move { from: 56, to: 57 };
    let state = action.apply(&state);

    assert!(!state.black_can_castle_queenside);
}

#[test]
fn moving_black_kingside_rook_removes_black_kingside_castle_rights_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
        Placement::new(White, Rook, 51),
    ]);
    state.black_can_castle_kingside = true;
    state.to_move = Black;

    let action = Move { from: 63, to: 62 };
    let state = action.apply(&state);

    assert!(!state.black_can_castle_kingside);
}

#[test]
fn moving_black_king_removes_black_castle_rights_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
        Placement::new(White, Rook, 51),
    ]);
    state.black_can_castle_queenside = true;
    state.black_can_castle_kingside = true;
    state.to_move = Black;

    let action = Move { from: 60, to: 61 };
    let state = action.apply(&state);

    assert!(!state.black_can_castle_kingside);
    assert!(!state.black_can_castle_queenside);
}

#[test]
fn pawn_threats_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 44),
        Placement::new(White, Pawn, 36),
    ]);
    // Pawns do threaten immediately forward diagonal squares
    assert!(color_threatens_square(White, 43, &state));
    assert!(color_threatens_square(White, 45, &state));
    assert!(color_threatens_square(Black, 35, &state));
    assert!(color_threatens_square(Black, 37, &state));
    // Pawns do not threaten immediately frontward
    assert!(!color_threatens_square(White, 44, &state));
    assert!(!color_threatens_square(Black, 36, &state));
}

#[test]
fn white_performs_en_passant_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 33),
        Placement::new(Black, Pawn, 50),
    ]);
    assert!(state.en_passant_square == None);

    // Two square advance
    let action = Move { from: 50, to: 34 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(42));

    // En Passant
    let action = EnPassant { with: 33 };
    let state = action.apply(&state);

    // The pawn that advanced two squares has been captured
    assert!(!state.squares[34].is_some());

}

#[test]
fn black_performs_en_passant_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
    ]);
    assert!(state.en_passant_square == None);

    // Two square advance
    let action = Move { from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));
    
    // En Passant
    let action = EnPassant { with: 29 };
    let state = action.apply(&state);

    // The pawn that advanced two squares has been captured
    assert!(!state.squares[28].is_some());
}

#[test]
fn white_knight_promotion_legality_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 55),
    ]);
    let action = Promotion { pawn_becomes: Knight, moving_from: 55, to: 63 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_bishop_promotion_legality_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 48),
    ]);
    let action = Promotion { pawn_becomes: Bishop, moving_from: 48, to: 56 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_rook_promotion_legality_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 49),
    ]);
    let action = Promotion { pawn_becomes: Rook, moving_from: 49, to: 57 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_queen_promotion_legality_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 50),
    ]);
    let action = Promotion { pawn_becomes: Queen, moving_from: 50, to: 58 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_bishop_promotion_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 12),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Bishop, moving_from: 12, to: 4 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_knight_promotion_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 11),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Knight, moving_from: 11, to: 3 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_rook_promotion_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 15),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Rook, moving_from: 15, to: 7 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_queen_promotion_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 14),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Queen, moving_from: 14, to: 6 };
    assert!(action.is_legal(&state));
}

#[test]
fn promotion_capture_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 14),
        Placement::new(White, Rook, 7),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Queen, moving_from: 14, to: 7 };
    assert!(action.is_legal(&state));
}

#[test]
fn en_passant_expires_after_move_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, King, 60),
    ]);
    // Two square advance by White
    let action = Move { from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

   // Black moves king
    let action = Move { from: 60, to: 61 };
    let state = action.apply(&state);

    // En-passant no longer legal
    assert!(state.en_passant_square == None);
}

#[test]
fn en_passant_expires_after_castle_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
    ]);
    // Two square advance by White
    let action = Move { from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

   // Black castles
    let action = Castle { direction: Kingside };
    let state = action.apply(&state);

    // En-passant no longer legal
    let action = Move { from: 29, to: 20 };
    assert!(state.en_passant_square == None);
}

#[test]
fn en_passant_expires_after_promotion_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, Pawn, 9),
    ]);

    assert!(state.to_move == White);
    // Two square advance by White
    let action = Move { from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

   // Black promotes
    let action = Promotion { pawn_becomes: Queen, moving_from: 9, to: 1 };
    let state = action.apply(&state);

    // En-passant no longer legal
    assert!(state.en_passant_square == None);
}

#[test]
fn en_passant_expires_after_en_passant_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 61),
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, King, 60),
    ]);
    // Two square advance by White
    let action = Move { from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

   // Black accepts en-passant
    let action = EnPassant { with: 60 };
    let state = action.apply(&state);

    // En-passant no longer legal
    assert!(state.en_passant_square == None);
}

#[test]
fn en_passant_expires_after_capture_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 61),
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, King, 60),
    ]);
    // Two square advance by White
    let action = Move { from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

   // Black capturess
    let action = Capture { on: 61, with: 60 };
    let state = action.apply(&state);

    // En-passant no longer legal
    assert!(state.en_passant_square == None);
}

#[test]
fn to_move_switches_after_move_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 8),
        Placement::new(Black, Pawn, 48),
    ]);
    // Advance by White
    let action = Move { from: 8, to: 16 };
    let state = action.apply(&state);
    assert!(state.to_move == Black);

    // Advance by Black 
    let action = Move { from: 48, to: 40 };
    let state = action.apply(&state);
    assert!(state.to_move == White);
}

#[test]
fn to_move_switches_after_promotion_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 55),
        Placement::new(Black, Pawn, 8),
    ]);
    // Promotion by White
    let action = Promotion { pawn_becomes: Queen, moving_from: 55, to: 63 };
    let state = action.apply(&state);
    assert!(state.to_move == Black);

    // Promotion by Black
    let action = Promotion { pawn_becomes: Queen, moving_from: 8, to: 1 };
    let state = action.apply(&state);
    assert!(state.to_move == White);
}

#[test]
fn to_move_switches_after_castle_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 56),
    ]);
    // Castle by White
    let action = Castle { direction: Kingside };
    let state = action.apply(&state);
    assert!(state.to_move == Black);

    // Castle by Black
    let action = Castle { direction: Queenside };
    let state = action.apply(&state);
    assert!(state.to_move == White);
}

#[test]
fn legal_actions_includes_moves_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
        Placement::new(Black, King, 60),
    ]);   
    state.to_move = Black;
    let actions = legal_actions(&state);
    assert!(actions.iter().any(|action| {
        action.name() == "Move"
    }));
}

#[test]
fn legal_actions_includes_promotions_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Pawn, 49),
        Placement::new(Black, King, 63),
    ]);   

    let actions = legal_actions(&state);
    assert!(actions.iter().any(|action| {
        action.name() == "Promotion"
    }));
}

#[test]
fn legal_actions_includes_all_legal_castles_by_white_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
        Placement::new(White, Rook, 7),
        Placement::new(Black, King, 63),
    ]);   
    state.white_can_castle_kingside = true;
    state.white_can_castle_queenside = true;

    let actions = legal_actions(&state);
    let legal_castles = actions.iter().filter(|action| {
        action.name() == "Castle"
    }).collect::<Vec<_>>();

    assert_eq!(2, legal_castles.len());
}

#[test]
fn legal_actions_includes_all_legal_castles_by_black_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
        Placement::new(Black, Rook, 56),
    ]);   
    state.black_can_castle_kingside = true;
    state.black_can_castle_queenside = true;
    state.to_move = Black;

    let actions = legal_actions(&state);
    let legal_castles = actions.iter().filter(|action| {
        action.name() == "Castle"
    }).collect::<Vec<_>>();

    assert_eq!(2, legal_castles.len());
}

#[test]
fn legal_actions_includes_all_legal_en_passants_by_white_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 33),
        Placement::new(White, Pawn, 35),
        Placement::new(Black, Pawn, 50),
    ]);
    state.to_move = Black;
    assert!(state.en_passant_square == None);

    // Two square advance
    let action = Move { from: 50, to: 34 };
    assert!(action.is_legal(&state));

    // EnPassant is now available
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(42));

    // Legal Actions include EnPassant
    let actions = legal_actions(&state);
    let legal_en_passants= actions.iter().filter(|action| {
        action.name() == "EnPassant"
    }).collect::<Vec<_>>();

    assert_eq!(2, legal_en_passants.len());
}

#[test]
fn legal_actions_includes_all_legal_en_passants_by_black_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 27),
        Placement::new(Black, Pawn, 29),
        Placement::new(White, Pawn, 12),
    ]);
    assert!(state.en_passant_square == None);

    // Two square advance
    let action = Move { from: 12, to: 28 };
    assert!(action.is_legal(&state));

    // EnPassant is now available
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

    // Legal Actions include EnPassant
    let actions = legal_actions(&state);
    let legal_en_passants= actions.iter().filter(|action| {
        action.name() == "EnPassant"
    }).collect::<Vec<_>>();

    assert_eq!(2, legal_en_passants.len());
}

#[test]
fn legal_actions_includes_all_legal_captures_by_white_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 33),
        Placement::new(White, Rook, 43),
        Placement::new(Black, Pawn, 40),
    ]);

    let actions = legal_actions(&state);
    let legal_captures = actions.iter().filter(|action| {
        action.name() == "Capture"
    }).collect::<Vec<_>>();

    assert_eq!(2, legal_captures.len());
}

#[test]
fn legal_actions_includes_all_legal_captures_by_black_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 33),
        Placement::new(Black, Pawn, 40),
        Placement::new(Black, Knight, 50),
    ]);
    state.to_move = Black;

    let actions = legal_actions(&state);
    let legal_captures = actions.iter().filter(|action| {
        action.name() == "Capture"
    }).collect::<Vec<_>>();

    assert_eq!(2, legal_captures.len());
}

#[test]
fn no_legal_actions_in_checkmate_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, Rook, 0),
        Placement::new(Black, Rook, 8),
    ]);

    let actions = legal_actions(&state);
    assert_eq!(0, actions.iter().len());
}

#[test]
fn white_promotion_to_bishop_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Pawn, 48),
        Placement::new(Black, King, 60),
    ]);   
    let action = Promotion { pawn_becomes: Bishop, moving_from: 48, to: 56 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_promotion_to_knight_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Pawn, 48),
        Placement::new(Black, King, 60),
    ]);   
    let action = Promotion { pawn_becomes: Knight, moving_from: 48, to: 56 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_promotion_to_rook_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Pawn, 49),
        Placement::new(Black, King, 60),
    ]);   
    let action = Promotion { pawn_becomes: Rook, moving_from: 49, to: 57 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_promotion_to_queen_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Pawn, 50),
        Placement::new(Black, King, 60),
    ]);   
    let action = Promotion { pawn_becomes: Queen, moving_from: 50, to: 58 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_promotion_to_bishop_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, Pawn, 8),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Bishop, moving_from: 8, to: 0 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_promotion_to_knight_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, Pawn, 15),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Knight, moving_from: 15, to: 7 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_promotion_to_rook_test() {
    let mut state = GameState::with_placements(vec![ Placement::new(White, King, 4),
        Placement::new(Black, Pawn, 14),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Rook, moving_from: 14, to: 6 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_promotion_to_queen_test() { let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, Pawn, 13),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Queen, moving_from: 13, to: 5 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_cant_move_into_check_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Pawn, 42),
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 0),
    ]);   
    let action = Move { from: 42, to: 50 };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_cant_promote_into_check_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Pawn, 50),
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 0),
    ]);   
    let action = Promotion { pawn_becomes: Queen, moving_from: 50, to: 58 };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_cant_move_into_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Queen, 48),
        Placement::new(Black, King, 60),
    ]);   
    state.to_move = Black;
    let action = Move { from: 60, to: 52 };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_cant_promote_into_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Queen, 56),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 11),
    ]);   
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Queen, moving_from: 11, to: 3 };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_pawn_can_capture_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 28),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 37),
    ]);

    let action = Capture { on: 37, with: 28 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_pawn_cant_capture_vertically_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 28),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 36),
    ]);
    let action = Capture { on: 36, with: 28 };
    assert!(!action.is_legal(&state));
}

#[test]
fn black_pawn_can_capture_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 28),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 37),
    ]);

    state.to_move = Black;
    let action = Capture { on: 28, with: 37 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_pawn_cant_capture_vertically_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 28),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 36),
    ]);
    state.to_move = Black;
    let action = Capture { on: 28, with: 36 };
    assert!(!action.is_legal(&state));
}

#[test]
fn white_bishop_can_capture_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 19),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 37),
    ]);

    let action = Capture { on: 37, with: 19 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_bishop_can_capture_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 19),
        Placement::new(Black, King, 60),
        Placement::new(Black, Bishop, 37),
    ]);

    state.to_move = Black;
    let action = Capture { on: 19, with: 37 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_knight_can_capture_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Knight, 20),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 37),
    ]);

    let action = Capture { on: 37, with: 20 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_knight_can_capture_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 20),
        Placement::new(Black, King, 60),
        Placement::new(Black, Knight, 37),
    ]);

    state.to_move = Black;
    let action = Capture { on: 20, with: 37 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_rook_can_capture_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Rook, 21),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 37),
    ]);

    let action = Capture { on: 37, with: 21 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_rook_can_capture_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 20),
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 36),
    ]);

    state.to_move = Black;
    let action = Capture { on: 20, with: 36 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_queen_can_capture_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Queen, 19),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 37),
    ]);

    let action = Capture { on: 37, with: 19 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_queen_can_capture_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 20),
        Placement::new(Black, King, 60),
        Placement::new(Black, Queen, 36),
    ]);

    state.to_move = Black;
    let action = Capture { on: 20, with: 36 };
    assert!(action.is_legal(&state));
}

#[test]
fn white_king_can_capture_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 36),
        Placement::new(Black, King, 60),
        Placement::new(Black, Pawn, 37),
    ]);

    let action = Capture { on: 37, with: 36 };
    assert!(action.is_legal(&state));
}

#[test]
fn black_king_can_capture_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 20),
        Placement::new(Black, King, 28),
    ]);

    state.to_move = Black;
    let action = Capture { on: 20, with: 28 };
    assert!(action.is_legal(&state));
}

#[test]
fn pawn_move_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 20),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 20, to: 28 };
    assert!(action.is_legal(&state));
    assert_eq!("e4", action.as_algebraic_notation(&state));
}

#[test]
fn bishop_move_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 20),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 20, to: 38 };
    assert!(action.is_legal(&state));
    assert_eq!("Bg5", action.as_algebraic_notation(&state));
}

#[test]
fn knight_move_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Knight, 20),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 20, to: 37 };
    assert!(action.is_legal(&state));
    assert_eq!("Nf5", action.as_algebraic_notation(&state));
}

#[test]
fn rook_move_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Rook, 20),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 20, to: 18 };
    assert!(action.is_legal(&state));
    assert_eq!("Rc3", action.as_algebraic_notation(&state));
}

#[test]
fn queen_move_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Queen, 36),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 36, to: 39 };
    assert!(action.is_legal(&state));
    assert_eq!("Qh5", action.as_algebraic_notation(&state));
}

#[test]
fn king_move_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 7, to: 6 };
    assert!(action.is_legal(&state));
    assert_eq!("Kg1", action.as_algebraic_notation(&state));
}

#[test]
fn move_algebraic_notation_with_ambiguous_file_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 19),
        Placement::new(White, Bishop, 21),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 19, to: 28 };
    assert!(action.is_legal(&state));
    assert_eq!("Bde4", action.as_algebraic_notation(&state));
}

#[test]
fn move_algebraic_notation_with_ambiguous_rank_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 19),
        Placement::new(White, Bishop, 35),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 19, to: 28 };
    assert!(action.is_legal(&state));
    assert_eq!("B3e4", action.as_algebraic_notation(&state));
}

#[test]
fn move_algebraic_notation_with_ambiguous_rank_and_file_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 19),
        Placement::new(White, Bishop, 21),
        Placement::new(White, Bishop, 35),
        Placement::new(Black, King, 60),
    ]);
    let action = Move { from: 19, to: 28 };
    assert!(action.is_legal(&state));
    assert_eq!("Bd3e4", action.as_algebraic_notation(&state));
}

#[test]
fn pawn_capture_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 35),
        Placement::new(Black, Pawn, 44),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Capture { on: 35, with: 44 };
    assert!(action.is_legal(&state));
    assert_eq!("exd5", action.as_algebraic_notation(&state));
}

#[test]
fn bishop_capture_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 35),
        Placement::new(Black, Bishop, 42),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Capture { on: 35, with: 42 };
    assert!(action.is_legal(&state));
    assert_eq!("Bxd5", action.as_algebraic_notation(&state));
}

#[test]
fn knight_capture_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Knight, 32),
        Placement::new(Black, Bishop, 42),
        Placement::new(Black, King, 60),
    ]);
    let action = Capture { on: 42, with: 32 };
    assert!(action.is_legal(&state));
    assert_eq!("Nxc6", action.as_algebraic_notation(&state));
}

#[test]
fn rook_capture_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Rook, 32),
        Placement::new(Black, Rook, 56),
        Placement::new(Black, King, 60),
    ]);
    let action = Capture { on: 56, with: 32 };
    assert!(action.is_legal(&state));
    assert_eq!("Rxa8", action.as_algebraic_notation(&state));
}

#[test]
fn queen_capture_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Queen, 32),
        Placement::new(Black, Rook, 56),
        Placement::new(Black, King, 60),
    ]);
    let action = Capture { on: 56, with: 32 };
    assert!(action.is_legal(&state));
    assert_eq!("Qxa8", action.as_algebraic_notation(&state));
}

#[test]
fn king_capture_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Pawn, 20),
        Placement::new(Black, King, 21),
    ]);
    state.to_move = Black;
    let action = Capture { on: 20, with: 21 };
    assert!(action.is_legal(&state));
    assert_eq!("Kxe3", action.as_algebraic_notation(&state));
}

#[test]
fn capture_algebraic_notation_with_ambiguous_file_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 35),
        Placement::new(Black, Rook, 32),
        Placement::new(Black, Rook, 38),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Capture { on: 35, with: 32 };
    assert!(action.is_legal(&state));
    assert_eq!("Raxd5", action.as_algebraic_notation(&state));
}

#[test]
fn capture_algebraic_notation_with_ambiguous_rank_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 7),
        Placement::new(White, Bishop, 35),
        Placement::new(Black, Queen, 43),
        Placement::new(Black, Queen, 27),
        Placement::new(Black, King, 60),
    ]);
    state.to_move = Black;
    let action = Capture { on: 35, with: 27 };
    assert!(action.is_legal(&state));
    assert_eq!("Q4xd5", action.as_algebraic_notation(&state));
}

#[test]
fn en_passant_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 33),
        Placement::new(Black, Pawn, 50),
    ]);
    state.to_move = Black;

    // Two square advance by black
    let action = Move { from: 50, to: 34 };
    let state = action.apply(&state);

    // En Passant by white
    let action = EnPassant { with: 33 };
    assert!(action.is_legal(&state));
    assert_eq!("bxc6", action.as_algebraic_notation(&state));
}

#[test]
fn kingside_castle_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    assert_eq!("O-O", action.as_algebraic_notation(&state));
}

#[test]
fn queenside_castle_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    assert_eq!("O-O-O", action.as_algebraic_notation(&state));
}

#[test]
fn promotion_to_bishop_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 55),
    ]);
    let action = Promotion { pawn_becomes: Bishop, moving_from: 55, to: 63 };
    assert_eq!("h8B", action.as_algebraic_notation(&state));
}

#[test]
fn promotion_to_knight_algebraic_notation_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 54),
    ]);
    let action = Promotion { pawn_becomes: Knight, moving_from: 54, to: 62 };
    assert_eq!("g8N", action.as_algebraic_notation(&state));
}

#[test]
fn promotion_to_rook_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 9),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Rook, moving_from: 9, to: 1 };
    assert_eq!("b1R", action.as_algebraic_notation(&state));
}

#[test]
fn promotion_to_queen_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 11),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Queen, moving_from: 11, to: 3 };
    assert_eq!("d1Q", action.as_algebraic_notation(&state));
}

#[test]
fn promotion_with_capture_algebraic_notation_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 11),
        Placement::new(White, Rook, 2),
    ]);
    state.to_move = Black;
    let action = Promotion { pawn_becomes: Queen, moving_from: 11, to: 2 };
    assert_eq!("dxc1Q", action.as_algebraic_notation(&state));
}

#[test]
fn is_checkmate_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Rook, 0),
        Placement::new(Black, Rook, 8),
        Placement::new(White, King, 4),
    ]);
    assert!(is_checkmate(&state));
}

#[test]
fn is_stalemate_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Rook, 57),
        Placement::new(Black, Rook, 15),
        Placement::new(White, King, 0),
    ]);
    println!("{}", state.to_string());
    assert!(is_stalemate(&state));
}

#[test]
fn gamestate_to_string_test() {
    let state = GameState::new();
    let expected = format!(
        "{}{}{}{}{}{}{}{}",

        "r n b q k b n r \n",
        "p p p p p p p p \n",
        ". . . . . . . . \n",
        ". . . . . . . . \n",
        ". . . . . . . . \n",
        ". . . . . . . . \n",
        "P P P P P P P P \n",
        "R N B Q K B N R \n"
    );

    let output = state.to_string();
    assert_eq!(output, expected);
}

#[test]
fn fen_notation_default_state_test() {
    let state = GameState::new();
    let expected = format!(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0"
    );

    let output = fen_notation(&state);
    assert_eq!(output, expected);
}

