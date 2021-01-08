use crate::{
  game::Game,
  matrix::Matrix,
  number_generator::NumberGenerator,
  sector::{Sector, SectorState},
};

pub struct GameBuilder {
  pub rows: usize,
  pub cols: usize,
  pub seed: u32,
  pub free_sector: Option<usize>,
  pub difficulty: u8,
}

impl GameBuilder {
  pub fn build(&self) -> Game {
    let mut sectors = Matrix::new(self.rows, self.cols);
    let mut num_gen = NumberGenerator::new(self.seed);
    let mines = self.generate_mine_idxs(&mut num_gen);

    for idx in 0..sectors.capacity() {
      let armed = mines.contains(&idx);

      let proximity = if armed {
        None
      } else {
        Some(
          sectors
            .adj_idxs(idx)
            .iter()
            .filter(|i| mines.contains(i))
            .count() as u8,
        )
      };

      sectors.push(Sector {
        armed,
        idx,
        proximity,
        state: SectorState::Hidden,
      });
    }

    Game::new(sectors)
  }

  fn generate_mine_idxs(&self, num_gen: &mut NumberGenerator) -> Vec<usize> {
    let total_sectors = self.rows * self.cols;
    // @TODO: Fix this abomination
    let total_mines =
      (((self.difficulty as f32 / 10 as f32) * total_sectors as f32) as u8) as usize;
    let excluded = self.free_sector.unwrap_or(total_mines + 1);
    let mut mine_idxs = Vec::with_capacity(total_mines);

    while mine_idxs.len() < total_mines {
      let idx = num_gen.rand_range(0, total_sectors as u32) as usize;

      if idx == excluded || mine_idxs.contains(&idx) {
        continue;
      }

      mine_idxs.push(idx);
    }

    mine_idxs
  }
}
