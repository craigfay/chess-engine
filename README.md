# About
* A Chess Engine built with Rust. **This is a work in progress**, and executing the crate with `cargo run` currently just runs the unit tests.

# Todo List
* Abstract horizontal/diagonal movement
* Replace Piece/Color enum's with chars.
* Allow state transitions
* Consolidate static variables
* Comment/Rename `algebraic_move()`
* Provide bit vector representation of game state
* Handle promotion
* Create a `potential_states()` as an alternative to `legal_moves()`
* Decide between a conventions chosen_move / m
* Move piece property to the beginning of Move constructors
* Maybe extract en-passant logic into it's own function
