use std::time::{Duration, Instant};

mod rules;
mod entities;
mod notation;
mod controller;

use rules::{
    color_threatens_square,
    color_is_checked,
    legal_moves,
    position_delta,
};

use entities::{
    GameState,
    Placement,
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
    Action,
    Move,
    Promotion,
    Castle,
    CastleDirection::{Kingside, Queenside},
};

use controller::{
    apply_move,
};

use notation::{algebraic, algebraic_move};

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

fn legal_moves_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, King, 60),
    ]);
    assert_eq!(5, legal_moves(&state).len());
}

fn legal_moves_no_kings_test() {
    // There can be legal moves even if state lacks Kings
    //
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 8),
        Placement::new(White, Pawn, 9),
    ]);
    assert_eq!(4, legal_moves(&state).len());
}


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

// Pawns should not be able to move sideways
fn pawn_movement_sideways_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 16),
    ]);
    let action = Move { piece: Pawn, from: 16, to: 17 };
    assert_eq!(false, action.is_legal(&state));
}


// Pawns should not be able to move more than two squares vertically
fn pawn_movement_too_far_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 18),
    ]);
    let action = Move { piece: Pawn, from: 18, to: 42 };
    assert_eq!(false, action.is_legal(&state));
}

// White pawns should be able to move 1 square up
fn pawn_movement_normal_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 22),
    ]);
    let action = Move { piece: Pawn, from: 22, to: 30 };
    assert_eq!(true, action.is_legal(&state));
}

// Pawns should not be able to move from an from square that has no pawn
fn pawn_movement_wrong_from_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 4),
    ]);
    let action = Move { piece: Pawn, from: 22, to: 30 };
    assert_eq!(false, action.is_legal(&state));
}

// Rooks should be able to travel horizontally
fn rook_movement_horizontal_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Rook, 35),
    ]);
    let action = Move { piece: Rook, from: 35, to: 32 };
    assert_eq!(true, action.is_legal(&state));
}

// Rooks should be able to travel vertically
fn rook_movement_vertical_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Rook, 35),
    ]);
    let action = Move { piece: Rook, from: 35, to: 3 };
    assert_eq!(true, action.is_legal(&state));
}


// Rooks should not be able to travel horizontally through other pieces
fn rook_movement_horizontal_obstruction_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Rook, 32),
        Placement::new(Black, Pawn, 33),
    ]);
    let action = Move { piece: Rook, from: 32, to: 36 };
    assert_eq!(false, action.is_legal(&state));
}

// Bishops should be able to travel diagonally up-left
fn bishop_movement_diagonal_up_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 22),
    ]);
    let action = Move { piece: Bishop, from: 22, to: 36 };
    assert_eq!(true, action.is_legal(&state));
}


// Bishops should be able to travel diagonally up-right
fn bishop_movement_diagonal_up_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 0),
    ]);
    let action = Move { piece: Bishop, from: 0, to: 36 };
    assert_eq!(true, action.is_legal(&state));
}

// Bishops should be able to travel diagonally down-left
fn bishop_movement_diagonal_down_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 27),
    ]);
    let action = Move { piece: Bishop, from: 27, to: 9 };
    assert_eq!(true, action.is_legal(&state));
}


// Bishops should be able to travel diagonally down-right
fn bishop_movement_diagonal_down_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 56),
    ]);
    let action = Move { piece: Bishop, from: 56, to: 42 };
    assert_eq!(true, action.is_legal(&state));
}


// Bishops shouldn't be able to wrap around the right edge of the state
fn bishop_movement_diagonal_right_edge_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 23),
    ]);
    let action = Move { piece: Bishop, from: 23, to: 41 };
    assert_eq!(false, action.is_legal(&state));
}


// Bishops shouldn't be able to wrap around the left edge of the state
fn bishop_movement_diagonal_left_edge_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Bishop, 24),
    ]);
    let action = Move { piece: Bishop, from: 24, to: 15 };
    assert_eq!(false, action.is_legal(&state));
}


fn knight_movement_two_up_one_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 45 };
    assert_eq!(true, action.is_legal(&state));
}


fn knight_movement_one_up_two_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 38 };
    assert_eq!(true, action.is_legal(&state));
}

fn knight_movement_two_up_one_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 43 };
    assert_eq!(true, action.is_legal(&state));
}


fn knight_movement_one_up_two_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 34 };
    assert_eq!(true, action.is_legal(&state));
}

fn knight_movement_two_down_one_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 13 };
    assert_eq!(true, action.is_legal(&state));
}

fn knight_movement_one_down_two_right_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 22 };
    assert_eq!(true, action.is_legal(&state));
}

fn knight_movement_two_down_one_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 11 };
    assert_eq!(true, action.is_legal(&state));
}

fn knight_movement_one_down_two_left_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Knight, 28),
    ]);
    let action = Move { piece: Knight, from: 28, to: 18 };
    assert_eq!(true, action.is_legal(&state));
}

// Queens should be able to move horizontally
fn queen_movement_horizontal_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Queen, 24),
    ]);
    let action = Move { piece: Queen, from: 24, to: 30 };
    assert_eq!(true, action.is_legal(&state));
}

// Queens should be able to move horizontally
fn queen_movement_vertical_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, Queen, 24),
    ]);
    let action = Move { piece: Queen, from: 24, to: 48 };
    assert_eq!(true, action.is_legal(&state));
}

// Queens should be able to move diagonally
fn queen_movement_diagonal_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Queen, 24),
    ]);
    let action = Move { piece: Queen, from: 24, to: 42 };
    assert_eq!(true, action.is_legal(&state));
}

// Kings should be able to move one square horizontally
fn king_movement_horizontal_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, King, 28),
    ]);
    let action = Move { piece: King, from: 28, to: 27 };
    assert!(action.is_legal(&state));
}

// Kings should be able to move one square vertically
fn king_movement_vertical_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, King, 28),
    ]);
    let action = Move { piece: King, from: 28, to: 20 };
    assert!(action.is_legal(&state));
}


// Kings should be able to move one square diagonally
fn king_movement_diagonal_test() {
    let state = GameState::with_placements(vec![
        Placement::new(Black, King, 24),
    ]);
    let action = Move { piece: King, from: 24, to: 33 };
    assert!(action.is_legal(&state));
}

fn algebraic_notation_to_index_test() {
    let ranks = ["1","2","3","4","5","6","7","8"];
    let files = ["a","b","c","d","e","f","g","h"];

    for (r, rank) in ranks.iter().enumerate() {
        for (f, file) in files.iter().enumerate() {
            let square = format!("{}{}", file, rank);
            let index = r * 8 + f;
            assert_eq!(algebraic(&square), Some(index));
        }
    }
}

fn algebraic_moves_white_pawn_one_forward_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 8),
    ]);
    let action = algebraic_move("a3", state);
    let expected = Some(Move { piece: Pawn, from: 8, to: 16 });
    assert_eq!(action, expected);
}

fn algebraic_moves_black_pawn_one_forward_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 48),
    ]);
    state.to_move = Black;
    let action = algebraic_move("a6", state);
    let expected = Some(Move { piece: Pawn, from: 48, to: 40 });
    assert_eq!(action, expected);
}

fn algebraic_moves_white_pawn_two_forward_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 8),
    ]);
    let action = algebraic_move("a4", state);
    let expected = Some(Move { piece: Pawn, from: 8, to: 24 });
    assert_eq!(action, expected);
}

fn algebraic_moves_black_pawn_two_forward_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 48),
    ]);
    state.to_move = Black;
    let action = algebraic_move("a5", state);
    let expected = Some(Move { piece: Pawn, from: 48, to: 32 });
    assert_eq!(action, expected);
}

fn algebraic_moves_white_pawn_rank_1_test() {
    // it should be impossible for white pawns to move to rank 1
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 8),
    ]);
    let action = algebraic_move("a1", state);
    assert_eq!(action, None);
}

fn algebraic_moves_white_pawn_rank_2_test() {
    // it should be impossible for white pawns to move to rank 2
    let mut state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 16),
    ]);
    let action = algebraic_move("a2", state);
    assert_eq!(action, None);
}

fn algebraic_moves_black_pawn_rank_7_test() {
    // it should be impossible for black pawns to move to rank 7
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 40),
    ]);
    state.to_move = Black;
    let action = algebraic_move("a7", state);
    assert_eq!(action, None);
}

fn algebraic_moves_black_pawn_rank_8_test() {
    // it should be impossible for black pawns to move to rank 8
    let mut state = GameState::with_placements(vec![
        Placement::new(Black, Pawn, 48),
    ]);
    state.to_move = Black;
    let action = algebraic_move("a8", state);
    assert_eq!(action, None);
}

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

fn color_threatens_square_test() {
    let state = GameState::new();
    assert!(color_threatens_square(White, 20, &state));
    assert!(color_threatens_square(Black, 44, &state));

    // Forward pawn moves are not threatening
    assert!(!color_threatens_square(White, 28, &state));
    assert!(!color_threatens_square(Black, 36, &state));
}

fn state_after_move_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 0),
        Placement::new(Black, Rook, 56),
    ]);   
    let action = Move { from: 0, to: 1, piece: King };
    let new_state = action.apply(&state);
    assert!(new_state.squares[1].unwrap().name == King);
    assert!(!new_state.squares[0].is_some());
}

fn cannot_move_into_check_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(Black, King, 59),
        Placement::new(Black, Pawn, 20),
    ]);   
    let action = Move { from: 4, to: 13, piece: King };
    assert!(!action.is_legal(&state));
}

fn white_kingside_castle_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 7),
    ]);
    state.white_can_castle_kingside = true;
    let action = Castle { direction: Kingside };
    assert!(action.is_legal(&state));
}
  
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

fn white_queenside_castle_legality_test() {
    let mut state = GameState::with_placements(vec![
        Placement::new(White, King, 4),
        Placement::new(White, Rook, 0),
    ]);
    state.white_can_castle_queenside = true;
    let action = Castle { direction: Queenside };
    assert!(action.is_legal(&state));
}
  
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

fn pawn_threats_test() {
    let mut state = GameState::with_placements(vec![
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

fn white_performs_en_passant_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 33),
        Placement::new(Black, Pawn, 50),
    ]);
    assert!(state.en_passant_square == None);

    // Two square advance
    let action = Move { piece: Pawn, from: 50, to: 34 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(42));

    // The pawn that advanced two squares has been captured
    let action = Move { piece: Pawn, from: 33, to: 42 };
    let state = action.apply(&state);
    assert!(!state.squares[34].is_some());
}

fn black_performs_en_passant_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
    ]);
    assert!(state.en_passant_square == None);

    // Two square advance
    let action = Move { piece: Pawn, from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

    // The pawn that advanced two squares has been captured
    let action = Move { piece: Pawn, from: 29, to: 20 };
    let state = action.apply(&state);
    assert!(!state.squares[28].is_some());
}

fn white_knight_promotion_legality_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 55),
    ]);
    let action = Promotion { pawn_becomes: Knight, moving_from: 55, to: 63 };
    assert!(action.is_legal(&state));
}

fn white_bishop_promotion_legality_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 48),
    ]);
    let action = Promotion { pawn_becomes: Bishop, moving_from: 48, to: 56 };
    assert!(action.is_legal(&state));
}

fn white_rook_promotion_legality_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 49),
    ]);
    let action = Promotion { pawn_becomes: Rook, moving_from: 49, to: 57 };
    assert!(action.is_legal(&state));
}

fn en_passant_expires_after_move_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, King, 60),
    ]);
    // Two square advance by White
    let action = Move { piece: Pawn, from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

   // Black moves king
    let action = Move { piece: Pawn, from: 12, to: 28 };
    let state = action.apply(&state);

    // En-passant no longer legal
    assert!(state.en_passant_square == None);
}

fn en_passant_expires_after_castle_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, King, 60),
        Placement::new(Black, Rook, 63),
    ]);
    // Two square advance by White
    let action = Move { piece: Pawn, from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));

   // Black castles
    let action = Castle { direction: Kingside };
    let state = action.apply(&state);

    // En-passant no longer legal
    let action = Move { piece: Pawn, from: 29, to: 20 };
    assert!(state.en_passant_square == None);
}

fn en_passant_expires_after_promotion_test() {
    let state = GameState::with_placements(vec![
        Placement::new(White, Pawn, 12),
        Placement::new(Black, Pawn, 29),
        Placement::new(Black, Pawn, 9),
    ]);

    assert!(state.to_move == White);
    // Two square advance by White
    let action = Move { piece: Pawn, from: 12, to: 28 };
    let state = action.apply(&state);
    assert!(state.en_passant_square == Some(20));
    assert!(state.to_move == Black);

   // Black promotes
    let action = Promotion { pawn_becomes: Queen, moving_from: 9, to: 1 };
    let state = action.apply(&state);
    assert!(state.to_move == White);

    // En-passant no longer legal
    assert!(state.en_passant_square == None);
}

fn main() {
    // Time tests
    let timer = Instant::now();

    new_gamestate_test();
    legal_moves_test();
    legal_moves_no_kings_test();
    position_delta_test();
    pawn_movement_sideways_test();
    pawn_movement_too_far_test();
    pawn_movement_normal_test();
    pawn_movement_wrong_from_test();
    rook_movement_horizontal_test();
    rook_movement_vertical_test();
    rook_movement_horizontal_obstruction_test();
    bishop_movement_diagonal_up_left_test();
    bishop_movement_diagonal_up_right_test();
    bishop_movement_diagonal_down_left_test();
    bishop_movement_diagonal_down_right_test();
    bishop_movement_diagonal_right_edge_test();
    bishop_movement_diagonal_left_edge_test();
    knight_movement_two_up_one_right_test();
    knight_movement_one_up_two_right_test();
    knight_movement_two_up_one_left_test();
    knight_movement_one_up_two_left_test();
    knight_movement_two_down_one_right_test();
    knight_movement_one_down_two_right_test();
    knight_movement_two_down_one_left_test();
    knight_movement_one_down_two_left_test();
    queen_movement_horizontal_test();
    queen_movement_vertical_test();
    queen_movement_diagonal_test();
    king_movement_horizontal_test();
    king_movement_vertical_test();
    king_movement_diagonal_test();
    algebraic_notation_to_index_test(); 
    algebraic_moves_white_pawn_one_forward_test(); 
    algebraic_moves_black_pawn_one_forward_test(); 
    algebraic_moves_white_pawn_two_forward_test(); 
    algebraic_moves_black_pawn_two_forward_test(); 
    algebraic_moves_white_pawn_rank_1_test(); 
    algebraic_moves_white_pawn_rank_2_test(); 
    algebraic_moves_black_pawn_rank_7_test(); 
    algebraic_moves_black_pawn_rank_8_test(); 
    color_is_checked_test();
    color_threatens_square_test();
    state_after_move_test();
    cannot_move_into_check_test();
    white_kingside_castle_legality_test();
    black_kingside_castle_legality_test();
    white_queenside_castle_legality_test();
    black_queenside_castle_legality_test();
    white_kingside_castle_aftermath_test();
    white_queenside_castle_aftermath_test();
    black_kingside_castle_aftermath_test();
    black_queenside_castle_aftermath_test();
    white_kingside_castle_obstruction_test();
    white_queenside_castle_obstruction_test();
    black_kingside_castle_obstruction_test();
    black_queenside_castle_obstruction_test();
    white_kingside_castle_out_of_check_test();
    white_kingside_castle_into_check_test();
    white_kingside_castle_through_check_test();
    white_queenside_castle_out_of_check_test();
    white_queenside_castle_into_check_test();
    white_queenside_castle_through_check_test();
    black_kingside_castle_out_of_check_test();
    black_kingside_castle_into_check_test();
    black_kingside_castle_through_check_test();
    black_queenside_castle_out_of_check_test();
    black_queenside_castle_into_check_test();
    black_queenside_castle_through_check_test();
    pawn_threats_test();
    white_performs_en_passant_test();
    black_performs_en_passant_test();
    white_knight_promotion_legality_test();
    white_bishop_promotion_legality_test();
    white_rook_promotion_legality_test();
    en_passant_expires_after_move_test();
    en_passant_expires_after_castle_test();
    en_passant_expires_after_promotion_test();

    let duration = timer.elapsed();
    println!("Tests finished in {:?}", duration);
}


