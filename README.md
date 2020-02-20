# About
* A Chess Engine built with Rust. **This is a work in progress**, and executing the crate with `cargo run` currently just runs the unit tests.

# Todo List
* Maybe Rename Move type as Action
* Abstract horizontal/diagonal movement
* Replace Piece/Color enum's with chars.
* Allow state transitions
* Use tuple constructors for Actions
* Consolidate static variables
* Comment/Rename `algebraic_move()`
* Provide bit vector representation of game state
* Handle en-passant
* Handle castling
* Handle promotion
* Create a `potential_states()` as an alternative to `legal_moves()`

