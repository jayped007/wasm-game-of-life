import { Universe } from "wasm-game-of-life";

// get 'pre' canvas element from index.html (centered)
const pre = document.getElementById("game-of-life-canvas");

// create Universe (via wasm)
const universe = Universe.new();

// iteration callback -- requestAnimationFrame every tick and update Universe
const renderLoop = () => {
  pre.textContent = universe.render(); // current universe -- rendered via Display trait from Rust wasm -- shows square boxes
  universe.tick(); // update Universe (seen on next tick), defined in Rust wasm

  requestAnimationFrame(renderLoop); // infinite loop

  // Do frames pile up? No.  This frame doesn't wait for next iteration of renderloop.  It requested it and its done.
};

// start rendering
requestAnimationFrame(renderLoop);
