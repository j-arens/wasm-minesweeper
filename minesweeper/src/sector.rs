use crate::{Action, Effect};

#[derive(PartialEq)]
pub enum SectorState {
  Flagged,
  Hidden,
  Questioned,
  Revealed,
}

impl SectorState {
  pub fn next(&self, action: Action) -> Option<Self> {
    match self {
      SectorState::Hidden => match action {
        Action::Primary => Some(SectorState::Revealed),
        Action::Secondary => Some(SectorState::Flagged),
      },
      SectorState::Flagged => Some(SectorState::Questioned),
      SectorState::Questioned => Some(SectorState::Hidden),
      _ => None,
    }
  }
}

pub struct Sector {
  pub armed: bool,
  pub idx: usize,
  pub proximity: Option<u8>,
  pub state: SectorState,
}

impl Sector {
  pub fn set_state(&mut self, state: SectorState) -> Effect {
    self.state = state;

    match self.state {
      SectorState::Flagged => Effect::Flag(self.idx),
      SectorState::Hidden => Effect::Hide(self.idx),
      SectorState::Questioned => Effect::Question(self.idx),
      SectorState::Revealed => match self.armed {
        true => Effect::Explode(self.idx),
        false => Effect::Reveal(self.idx, self.proximity.unwrap()),
      },
    }
  }
}
