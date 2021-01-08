use std::cmp;

/**
 * Really basic implementation of a psuedo-random number generator. In normal
 * circumstances it would be advised to use the `rand` crate. I played around
 * with `rand` briefly but I couldn't get it to work in the browser without
 * `wasm-bindgen` which I wanted to avoid since the entire point of this
 * exercise was to fill in my own knowledge gaps that `wasm-bindgen` was
 * glazing over.
 *
 * @see https://en.wikipedia.org/wiki/Linear_congruential_generator
 */

pub struct NumberGenerator {
  state: u32,
}

impl NumberGenerator {
  pub fn new(seed: u32) -> Self {
    Self { state: seed }
  }

  pub fn rand(&mut self) -> u32 {
    self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
    self.state %= 1 << 31;
    self.state
  }

  pub fn rand_range(&mut self, min: u32, max: u32) -> u32 {
    // lol bad
    cmp::max(min, self.rand() % max)
  }
}
