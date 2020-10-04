import { Universe, Cell } from "wasm-game-of-life";

// Import the WebAssembly memory
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC"; // grey
const DEAD_COLOR = "#FFFFFF"; // white
const ALIVE_COLOR = "#000000"; // black

// create Universe (via wasm)
const universe = Universe.new();
const width = universe.width(); // NOT Universe.width() !!!!
const height = universe.height();

// Give the canvas room for all cells and 1px border
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext("2d");

const getRowStartPos = (row) => row * (CELL_SIZE + 1) + 1;
const getColStartPos = (col) => col * (CELL_SIZE + 1) + 1;

const drawGrid = () => {
  // NOTE: ctx is external reference to 2d context w/in canvas
  // Algo:
  //   Draw equally-spaced horizontal lines, and equally-spaced vertical lines
  //   which criss-cross to form the grid.

  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  let end_col_pos = getColStartPos(height);
  for (let i = 0; i <= width; i++) {
    let row_pos = getRowStartPos(i);
    ctx.moveTo(row_pos, 0);
    ctx.lineTo(row_pos, end_col_pos);
  }

  // Horizontal lines.
  let end_row_pos = getRowStartPos(width);
  for (let j = 0; j <= height; j++) {
    let col_pos = getColStartPos(j);
    ctx.moveTo(0, col_pos);
    ctx.lineTo(end_row_pos, col_pos);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : ALIVE_COLOR;

      ctx.fillRect(
        getColStartPos(col),
        getRowStartPos(row),
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

// iteration callback -- requestAnimationFrame every tick and update Universe
const renderLoop = () => {
  universe.tick(); // update Universe

  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop); // infinite loop

  // Do frames pile up? No.  This frame doesn't wait for next iteration of renderloop.  It requested it and its done.
};

// start rendering
drawGrid();
drawCells();
requestAnimationFrame(renderLoop);
