
# About
* A chess gamestate engine for Rust! Designed as a utility for machine learning research for [perfect information](https://en.wikipedia.org/wiki/Complete_information) systems.

# Usage
* Include as a dependency in your Rust Crate:

```
# Cargo.toml
[dependencies]
chess-engine = { git = "https://github.com/craigfay/chess-engine" }
```

```
// main.rs

use chess_engine::{
  GameState,
  Color,
  PieceName,
  legal_actions,
  legal_next_states,
  is_checkmate,
  is_stalemate,
  relative_material_values, 
};

fn main() {
  let state = GameState::new();

  println!("{}", state.to_string());

  assert(false == is_checkmate(&state));
  assert(false == is_stalemate(&state));

  let available_actions = legal_actions(&state);
  let some_action = legal_actions[0];

  println!("{}", some_action.as_algebraic_notation());

  let state = some_action.apply(&state);

  println!("{}", state.to_string());
}

```

