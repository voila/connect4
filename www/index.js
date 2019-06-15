import { Board, Cell, Result } from "connect4";
import { memory } from "connect4/connect4_bg";

const CELL_SIZE = 60; // px
const GRID_COLOR = "#0000AA";
const RED_COLOR = "#FF0000";
const YELLOW_COLOR = "#FFFF00";
const EMPTY_COLOR = "#00F0FF";

// Construct the board
const board = Board.new();
const width = board.width();
const height = board.height();

const message = document.getElementById("message");
// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;
const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.beginPath();
  ctx.fillStyle = GRID_COLOR;
  ctx.fillRect(0, 0, width * CELL_SIZE, height * CELL_SIZE);
};

const getIndex = (row, column) => {
  return row * width + column;
};
const drawCells = () => {
  const cellsPtr = board.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.beginPath();
      ctx.fillStyle = EMPTY_COLOR;
      ctx.fillStyle = cells[idx] === Cell.Empty
        ? EMPTY_COLOR
        : cells[idx] === Cell.Yellow
          ? YELLOW_COLOR
          : RED_COLOR;
      let x = col * CELL_SIZE + CELL_SIZE / 2;
      let y = row * CELL_SIZE + CELL_SIZE / 2;
      ctx.arc(x, y, CELL_SIZE / 2 - 2, 0, 2 * Math.PI);
      ctx.fill();
    }
  }
}

function getCursorPosition(canvas, event) {
  var rect = canvas.getBoundingClientRect();
  var x = event.clientX - rect.left;
  var y = event.clientY - rect.top;
  //console.log("x: " + x + " y: " + y);
  return [x, y];
}

const showPlayer = () => {
  return player === Cell.Yellow ? "Yellow" : "Red";
}

const render = (s) => {
  message.innerHTML = s;
  drawGrid();
  drawCells();
};

const swapPlayer = (p) => {
  return p === Cell.Yellow ? Cell.Red : Cell.Yellow;
}

const handleClick = (e) => {
  let [x, y] = getCursorPosition(canvas, e);
  // here translate x,y to a cell 
  let row = Math.floor(y / CELL_SIZE);
  let column = Math.floor(x / CELL_SIZE);
  // play move
  let msg;
  let res = board.play(player, row, column);
  switch (res) {
    case Result.Win:
      // is game finished? announce winner, deregister click handler
      msg = showPlayer() + " has won!";
      canvas.onclick = null;
      break;
    case Result.Draw:
      msg = "It is a draw!";
      canvas.onclick = null;
      break;
    case Result.Unfinished:
      //console.log('unfinished')
      // if not let the other play
      player = swapPlayer(player);
      msg = showPlayer() + " turn to play";
      break;
    case Result.InvalidMove:
      msg = 'Invalid move';
      break;
  }
  // draw cells
  render(msg);
}

// Start the game
let player = Cell.Yellow; // player is global !!!
canvas.onclick = (e) => handleClick(e);
let msg = showPlayer() + " turn to play";
render(msg);