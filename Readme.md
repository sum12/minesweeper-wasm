WASM + RUST
===========

Minesweeper like game using wasm for game logic and js for view part.

[Demo](https://sum12.github.io/minesweeper-wasm/)


Learnings
=========

Rust's lib memory is accessible via the JS system. It feels like trying to access `unsafe` code.


This memory is the heap where wasm compiles Rust lib operates on. So given the pointer and size of 
any datastructure created by the lib is known, that data can be accessed via pointer arithmatic.


Here the game state is stored in a `Vec` of `struct Cell`. Each `Cell` is 2 words long and stores
the state of a "Cell". First word is actual `CellState::{Close, Open, Flag}` and Second word is a
`bool` indicating presence of a Mine. The `Vec` has `height x width` `Cell`s. So `Cell`  at `(x,y)`
is accessible via `vec[2*(y*height + x)]`


This is exactly the logic used by JS code to draw cells on the screen in [index.html](./index.html).
Rest of the logic for game is defined in [./src/lib.rs](./src/lib.rs)


License


- GPL
