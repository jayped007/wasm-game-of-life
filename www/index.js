import { Universe, Cell, InitialPattern } from "wasm-game-of-life";

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

const getRowStartPos = (row) => row * (CELL_SIZE + 1) + 1;
const getColStartPos = (col) => col * (CELL_SIZE + 1) + 1;

// Give the canvas room for all cells and 1px border
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = getRowStartPos(height);
canvas.width = getColStartPos(width);

const ctx = canvas.getContext("2d");

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

const findRowColOfClick = (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  // convert coords to row, col then toggle cell then redraw grid
  // NOTE: cells are CELL_SIZE+1 pixels wide and high (including border)
  // NOTE: coordinates we have (canvasLeft,Top) are pixel counts relative to upper LH corner

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

  return [row, col];
};

// Because we want to disable this event some of the time, it cant be anonymous
const canvas_click_event_listener = (evt) => {
  if (universe.is_mousedown()) {
    universe.set_mousedown_value(false); // only ignore once
    return;
  }
  // obtain coordinates where mouse was clicked
  const [row, col] = findRowColOfClick(evt);
  console.log("canvas click in (row,col) = (" + row + ", " + col + ")");
  universe.toggle_cell(row, col);
  drawGrid();
  drawCells();
};

// click event for canvas, toggle cell that was clicked on
canvas.addEventListener("click", canvas_click_event_listener);

canvas.addEventListener("mousedown", (evt) => {
  if (evt.shiftKey) {
    universe.set_mousedown_value(true); // cause associated click event to be ignored
    // Shift-click ==> create glider at (row,col)
    const [row, col] = findRowColOfClick(evt);
    console.log(
      "canvas mousedown shift-click in (row,col) = (" + row + ", " + col + ")"
    );
    pauseAction(); // no ticks until glider inserted
    playPauseButton.textContent = "▶";
    // create glider starting at (row,col)
    universe.set_cell_value(row, col, Cell.Alive); // first row of 3
    universe.set_cell_value(row, col + 1, Cell.Dead);
    universe.set_cell_value(row, col + 2, Cell.Alive);
    universe.set_cell_value(row + 1, col, Cell.Dead); // second row of 3
    universe.set_cell_value(row + 1, col + 1, Cell.Alive);
    universe.set_cell_value(row + 1, col + 2, Cell.Alive);
    universe.set_cell_value(row + 2, col, Cell.Dead); // third row of 3
    universe.set_cell_value(row + 2, col + 1, Cell.Alive);
    universe.set_cell_value(row + 2, col + 2, Cell.Dead);
    drawGrid();
    drawCells();
    restartAction();
  }
});

// iteration callback -- requestAnimationFrame every tick and update Universe
let animationId = null;

const renderLoop = () => {
  //debugger; // starts browser debugger, JS keyword, ECMAscript 1
  universe.tick(); // update Universe

  drawGrid();
  drawCells();

  animationId = requestAnimationFrame(renderLoop); // infinite loop
  // NOTE: animationId allows for stopping the iteration

  // Do frames pile up? No.  This frame doesn't wait for next iteration of renderloop.  It requested it and its done.
};

// isPaused() => Determine if game currently paused
const isPaused = () => animationId === null;

// Handle restart button

const restartButton = document.getElementById("restart-btn");

restartButton.addEventListener("click", (e) => {
  pauseAction();
  universe.reset_board();
  playPauseButton.textContent = "⏸";
  restartAction();
});

// Handle play/pause button

const playPauseButton = document.getElementById("play-pause-btn");

const play = () => {
  // restart game, set button text to pause indicator
  console.log("play()");
  playPauseButton.textContent = "⏸";
  restartAction();
};

const pause = () => {
  // pause game due to button click, set button text to play indicator
  console.log("pause()");
  playPauseButton.textContent = "▶";
  pauseAction();
};

const pauseAction = () => {
  // pause game
  cancelAnimationFrame(animationId);
  animationId = null;
};

const restartAction = () => {
  renderLoop();
};

// click event handler
playPauseButton.addEventListener("click", (event) => {
  if (isPaused()) {
    play();
  } else {
    pause();
  }
});

// start rendering
drawGrid();
drawCells();
play();
