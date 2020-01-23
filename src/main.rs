mod chess;
mod entities;

use chess::{
    GameRules,
    position_delta,
};

use entities::{
    GameBoard,
    Piece,
    PieceType,
    ActionType,
    Color,
    Move,
};

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
    let mut board = GameBoard::new();
    let pawn = Piece::new(Color::White, PieceType::Pawn);
    board.place_piece(pawn, 16);

    let chosen_move = Move {

        action: ActionType::Move,
        piece: PieceType::Pawn,
        origin: 16,
        destination: 17,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}


// Pawns should not be able to move more than two squares vertically
fn pawn_movement_too_far_test() {
    let mut board = GameBoard::new();
    let pawn = Piece::new(Color::White, PieceType::Pawn);
    board.place_piece(pawn, 18);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: PieceType::Pawn,
        origin: 18,
        destination: 42,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}

// White pawns should be able to move 1 square up
fn pawn_movement_normal_test() {
    let mut board = GameBoard::new();
    let pawn = Piece::new(Color::White, PieceType::Pawn);
    board.place_piece(pawn, 22);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: PieceType::Pawn,
        origin: 22,
        destination: 30,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


// Pawns should not be able to move from an origin square that has no pawn
fn pawn_movement_wrong_origin_test() {
    let mut board = GameBoard::new();
    let pawn = Piece::new(Color::White, PieceType::Pawn);
    board.place_piece(pawn, 4);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: PieceType::Pawn,
        origin: 22,
        destination: 30,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}


fn main() {
    position_delta_test();
    pawn_movement_sideways_test();
    pawn_movement_too_far_test();
    pawn_movement_normal_test();
    pawn_movement_wrong_origin_test();
}

