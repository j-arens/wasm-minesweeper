# WASM Minesweeper

This repo serves as an archive of my first real attempt to use Rust in the browser via WASM. I did not have any particular goals in mind other than to simply learn, so keep in mind that this is probably a poor use case of WASM. I decided not to utilize [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) because I wanted to understand how things work at a low level, however in the real world I would highly recommened using it.

The basic idea is that all of the Minesweeper game logic is implemented in Rust and compiled to WASM and the user interface is plain HTML/CSS/JavaScript. From there the JavaScript simply acts as a proxy between the DOM and the instantiated WebAssembly module.

## To run yourself

Requirements:

- Rust
- Cargo
- Node
- Yarn

Clone the repo locally and run `yarn` to install the required dependencies. Afterwards run `yarn start` to build the code and open a webpage. To build optimized code simply prefix the start command with `NODE_ENV=production`.
