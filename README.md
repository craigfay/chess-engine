
# About
* A Chess Engine built with Rust. **This is a work in progress**, and executing the crate with `cargo run` currently just runs the unit tests.

# Todo List
* Abstract horizontal/diagonal movement
* Consolidate static variables
* Comment/Rename `algebraic_move()`
* Provide bit vector representation of game state
* Create a `potential_states()` as an alternative to `legal_moves()`
* Move piece property to the beginning of Move constructors
* Handle/Test generic legality checks for each action (is correct color, piece exists, etc..)
* Refactor with tuple matching
* Test that to_move has flipped after each applied action
* Should actions be responsible for changing the turn?
* Use non-deterministic tests with random move selection to test for states that should never exist.
* prevent attacking with the wrong color
* Make move_is_psuedo_legal() more generic, and just take a piecename, origin, and destination.
* Use rays to check legality
* Use a controller/display modules to play a game in the command line
* Fix diagonal_move_is_obstructed()
