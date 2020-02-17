
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


