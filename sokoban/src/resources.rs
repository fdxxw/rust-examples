use std::{fmt::Display, time::Duration};

use ggez::event;
use specs::World;

use crate::{events::Event, audio::AudioStore};

#[derive(Default)]
pub struct InputQueue {
    pub key_pressed: Vec<event::KeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(Time::default());
    world.insert(EventQueue::default());
    world.insert(AudioStore::default());
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub move_count: u32,
}
pub enum GameplayState {
    Playing,
    Won,
}

impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GameplayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}


#[derive(Default)]
pub struct Time {
  pub delta: Duration
}
#[derive(Default)]
pub struct EventQueue {
  pub events: Vec<Event>
}