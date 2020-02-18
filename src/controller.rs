// This module's single responsibility is to control the flow of a chess game
// using entities and rules.

use crate::entities::{
    GameState,
    Move,
};

use crate::rules::{
    move_is_legal,
};

pub fn apply_move(m: &Move, state: &mut GameState) {
    if false == move_is_legal(m, state) {
        panic!("Cannot apply illegal move: {:?}", m);
    }

    state.squares[m.destination] = state.squares[m.origin];
}

