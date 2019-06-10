### Setup

$ cargo generate --git https://github.com/rustwasm/wasm-pack-template
🤷  Project Name: connect4
🔧   Creating project called `connect4`...
✨   Done! New project created ~/WebAssembly/connect4

$ wasm-pack build
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
    Updating crates.io index
  Downloaded syn v0.15.35
  Downloaded bumpalo v2.4.3
   Compiling proc-macro2 v0.4.30
   Compiling unicode-xid v0.1.0
   Compiling syn v0.15.35
   Compiling wasm-bindgen-shared v0.2.45
   Compiling cfg-if v0.1.9
   Compiling bumpalo v2.4.3
   Compiling lazy_static v1.3.0
   Compiling wasm-bindgen v0.2.45
   Compiling log v0.4.6
   Compiling quote v0.6.12


$ npm init wasm-app www
npx: installed 1 in 3.044s
🦀 Rust + 🕸 Wasm = ❤

edit www/package.json

"dependencies": {
    "connect4": "file:../pkg"
  },

$ npm install
added 1 package and audited 8970 packages in 5.231s
found 0 vulnerabilities

$ npm run start

> create-wasm-app@0.1.0 start ~/WebAssembly/connect4/www
> webpack-dev-server

Project is running at http://localhost:8080/  

### Developing the game

Javascript will manage the turn taking

The wasm side will

* store the board state (take inspiration from game of life in rust-wasm book)
* validate a move, 
* update the board after a move, 
* display a board
* decide whether the game is won or drawn


### One-player and Minimax 