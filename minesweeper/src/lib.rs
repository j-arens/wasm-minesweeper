extern crate wee_alloc;

mod game;
mod game_builder;
mod matrix;
mod number_generator;
mod sector;

use crate::game::Game;
use crate::game_builder::GameBuilder;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub enum Action {
  Primary,
  Secondary,
}

pub struct Selection {
  pub action: Action,
  pub idx: usize,
}

pub enum Effect {
  Explode(usize),
  Flag(usize),
  Hide(usize),
  Lost,
  Question(usize),
  Reveal(usize, u8),
  Won,
}

extern "C" {
  fn explode(idx: i32, is_catalyst: i32);
  fn flag(idx: i32);
  fn hide(idx: i32);
  fn lost();
  fn question(idx: i32);
  fn reveal(idx: i32, proximity: i32);
  fn won();
}

#[no_mangle]
pub extern "C" fn game_builder_build(
  rows: i32,
  cols: i32,
  difficulty: i32,
  free_sector: i32,
  action: i32,
  random_num: i32,
) -> i32 {
  let builder = GameBuilder {
    rows: rows as usize,
    cols: cols as usize,
    free_sector: Some(free_sector as usize),
    difficulty: difficulty as u8,
    seed: random_num as u32,
  };

  let game = builder.build();
  let game = Box::into_raw(Box::new(game)) as i32;

  game_on_select(game, free_sector, action);

  game
}

#[no_mangle]
pub extern "C" fn game_on_select(game: i32, selected_idx: i32, action: i32) {
  let game = game as *mut Game;
  let game = unsafe { &mut *game };

  let selection = Selection {
    action: match action {
      0 => Action::Primary,
      _ => Action::Secondary,
    },
    idx: selected_idx as usize,
  };

  let effects = game.on_select(selection);

  if effects.is_none() {
    return;
  }

  unsafe {
    for effect in effects.unwrap() {
      match effect {
        Effect::Explode(idx) => {
          let is_catalyst = if idx == selected_idx as usize { 1 } else { 0 };
          explode(idx as i32, is_catalyst);
        }
        Effect::Flag(idx) => flag(idx as i32),
        Effect::Hide(idx) => hide(idx as i32),
        Effect::Lost => lost(),
        Effect::Question(idx) => question(idx as i32),
        Effect::Reveal(idx, proximity) => reveal(idx as i32, proximity as i32),
        Effect::Won => won(),
      }
    }
  }
}
