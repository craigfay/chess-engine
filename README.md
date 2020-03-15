
# About
* A Chess Engine built with Rust. **This is a work in progress**, and executing the crate with `cargo run` currently just runs the unit tests.

# Todo List
* Abstract horizontal/diagonal movement
* Provide bit vector representation of game state
* Create a `potential_states()` as an alternative to `legal_moves()`
* Handle/Test generic legality checks for each action (is correct color, piece exists, etc..)
* Refactor with tuple matching
* Test that to_move has flipped after each applied action
* Should actions be responsible for changing the turn? (Probably)
* Use non-deterministic tests with random move selection to test for states that should never exist.
* Use rays to check legality
* Use a controller/display modules to play a game in the command line
* Test captures with incorrect attacker/defender colors
* Implement and Test `as_algebraic_notation()` for each action type.
* Add search helper function to find the square that a piece is on.
* Test that pawns can't capture vertically
* Handle Promotions that are also Captures
* Create Algebraic equivalents to the actions that use strings instead of usize.
