use crate::{
  matrix::Matrix,
  sector::{Sector, SectorState},
  Effect, Selection,
};

pub struct Game {
  sectors: Matrix<Sector>,
}

impl Game {
  pub fn new(sectors: Matrix<Sector>) -> Self {
    Self { sectors }
  }

  pub fn on_select(&mut self, selection: Selection) -> Option<Vec<Effect>> {
    let sector = self.sectors.get_element_mut(selection.idx)?;
    let next_state = sector.state.next(selection.action)?;
    let mut effects: Vec<Effect>;

    match next_state {
      SectorState::Revealed => match sector.armed {
        true => {
          effects = self.reveal_mines();
          effects.push(Effect::Lost);
          return Some(effects);
        }
        false => effects = self.sweep(selection.idx),
      },
      _ => effects = vec![sector.set_state(next_state)],
    }

    if self.is_won() {
      effects.push(Effect::Won);
    }

    Some(effects)
  }

  fn is_won(&self) -> bool {
    self
      .sectors
      .iter()
      .filter(|sector| {
        if sector.state == SectorState::Hidden {
          if !sector.armed {
            return true;
          }
        }

        false
      })
      .count()
      == 0
  }

  fn reveal_mines(&mut self) -> Vec<Effect> {
    self
      .sectors
      .iter_mut()
      .filter_map(|sector| match sector.armed {
        true => Some(sector.set_state(SectorState::Revealed)),
        false => None,
      })
      .collect()
  }

  fn sweep(&mut self, idx: usize) -> Vec<Effect> {
    let mut effects = Vec::new();
    let sector = self.sectors.get_element_mut(idx).unwrap();

    if sector.state != SectorState::Hidden {
      return effects;
    }

    effects.push(sector.set_state(SectorState::Revealed));

    if sector.proximity.unwrap() == 0 {
      for adj_idx in self.sectors.adj_idxs(idx).iter() {
        effects.append(&mut self.sweep(*adj_idx));
      }
    }

    effects
  }
}
