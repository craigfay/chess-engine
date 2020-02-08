mod chess;
mod entities;

use chess::{
    GameRules,
    position_delta,
    square_as_algebraic,
};

use entities::{
    GameBoard,
    Placement,
    Piece,
    PieceType::{
        Pawn,
        Rook,
        Bishop,
        Knight,
    },
    ActionType,
    Color::{White, Black},
    Move,
};

fn gameboard_with_placements_test() {
    let board = GameBoard::with_placements(vec![
        Placement::new(Black, Pawn, 8),
        Placement::new(Black, Pawn, 9),
    ]);
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
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Pawn, 16),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Pawn,
        origin: 16,
        destination: 17,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}


// Pawns should not be able to move more than two squares vertically
fn pawn_movement_too_far_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Pawn, 18),
    ]);


    let chosen_move = Move {
        action: ActionType::Move,
        piece: Pawn,
        origin: 18,
        destination: 42,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}

// White pawns should be able to move 1 square up
fn pawn_movement_normal_test() {

    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Pawn, 22),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Pawn,
        origin: 22,
        destination: 30,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


// Pawns should not be able to move from an origin square that has no pawn
fn pawn_movement_wrong_origin_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Pawn, 4),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Pawn,
        origin: 22,
        destination: 30,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}

// Rooks should be able to travel horizontally
fn rook_movement_horizontal_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Rook, 35),
    ]);


    let chosen_move = Move {
        action: ActionType::Move,
        piece: Rook,
        origin: 35,
        destination: 32,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}

// Rooks should be able to travel vertically
fn rook_movement_vertical_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Rook, 35),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Rook,
        origin: 35,
        destination: 3,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


// Rooks should not be able to travel horizontally through other pieces
fn rook_movement_horizontal_obstruction_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Rook, 32),
        Placement::new(Black, Pawn, 33),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Rook,
        origin: 32,
        destination: 36,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}

// Bishops should be able to travel diagonally up-left
fn bishop_movement_diagonal_up_left_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Bishop, 22),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Bishop,
        origin: 22,
        destination: 36,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


// Bishops should be able to travel diagonally up-right
fn bishop_movement_diagonal_up_right_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Bishop, 0),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Bishop,
        origin: 0,
        destination: 36,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}

// Bishops should be able to travel diagonally down-left
fn bishop_movement_diagonal_down_left_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Bishop, 27),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Bishop,
        origin: 27,
        destination: 9,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


// Bishops should be able to travel diagonally down-right
fn bishop_movement_diagonal_down_right_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Bishop, 56),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Bishop,
        origin: 56,
        destination: 42,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


// Bishops shouldn't be able to wrap around the right edge of the board
fn bishop_movement_diagonal_right_edge_test() {
    let mut board = GameBoard::with_placements(vec![
        Placement::new(White, Bishop, 23),
    ]);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Bishop,
        origin: 23,
        destination: 41,
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}


// Bishops shouldn't be able to wrap around the left edge of the board
fn bishop_movement_diagonal_left_edge_test() {
    let mut board = GameBoard::new();
    let bishop = Piece::new(White, Bishop);
    board.place_piece(bishop, 24);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Bishop,
        origin: 24,
        destination: 15
    };

    assert_eq!(false, GameRules::can_move(chosen_move, board));
}


fn knight_movement_two_up_one_right_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(Black, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 45,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


fn knight_movement_one_up_two_right_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(Black, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 38,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}

fn knight_movement_two_up_one_left_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(White, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 43,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


fn knight_movement_one_up_two_left_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(White, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 34,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}

fn knight_movement_two_down_one_right_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(Black, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 13,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}

fn knight_movement_one_down_two_right_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(Black, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 22,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}

fn knight_movement_two_down_one_left_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(Black, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 11,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}

fn knight_movement_one_down_two_left_test() {
    let mut board = GameBoard::new();
    let knight = Piece::new(Black, Knight);
    board.place_piece(knight, 28);

    let chosen_move = Move {
        action: ActionType::Move,
        piece: Knight,
        origin: 28,
        destination: 18,
    };

    assert_eq!(true, GameRules::can_move(chosen_move, board));
}


fn square_as_algebraic_test() {
    assert_eq!("a1", square_as_algebraic(0));
    assert_eq!("b1", square_as_algebraic(1));
    assert_eq!("a2", square_as_algebraic(8));
}

fn main() {
    gameboard_with_placements_test();
    position_delta_test();
    pawn_movement_sideways_test();
    pawn_movement_too_far_test();
    pawn_movement_normal_test();
    pawn_movement_wrong_origin_test();
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
    square_as_algebraic_test(); 
}

