//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn will_pass_one_plus_one_eq_two() {
    assert_eq!(1 + 1, 2);
}

use wasm_game_of_life::Universe;

// cfg(test) -- initialize environment for testing
// ==>
// We'll want one for our input spaceship that we'll call the tick function
// on and we'll want the expected spaceship we will get after one tick.
// We picked the cells that we want to initialize as Alive to create our
// spaceship in the input_spaceship function. The position of the spaceship
// in the expected_spaceship function after the tick of the input_spaceship
// was calculated manually. You can confirm for yourself that the cells of
// the input spaceship after one tick is the same as the expected spaceship.

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
    universe
}

// Now we will write the implementation for our test_tick function. First, we
// create an instance of our input_spaceship() and our expected_spaceship().
// Then, we call tick on the input_universe. Finally, we use the assert_eq!
// macro to call get_cells() to ensure that input_universe and expected_universe
// have the same Cell array values. We add the #[wasm_bindgen_test] attribute
// to our code block so we can test our Rust-generated WebAssembly code and
// use wasm-pack test to test the WebAssembly code.

#[wasm_bindgen_test]
pub fn test_tick() {
    // Let's create a smaller Universe with a small spaceship to test!
    let mut input_universe = input_spaceship();

    // This is what our spaceship should look like
    // after one tick in our universe.
    let expected_universe = expected_spaceship();

    // Call `tick` and then see if the cells in the `Universe`s are the same.
    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}