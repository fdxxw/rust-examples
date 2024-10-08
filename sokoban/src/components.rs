use std::fmt::Display;

use specs::{Component, NullStorage, VecStorage, World, WorldExt};

#[derive(Clone, Debug, Copy, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub paths: Vec<String>,
}

impl Renderable {
    pub fn new_static(path: String) -> Self {
        Self { paths: vec![path] }
    }
    pub fn new_animated(paths: Vec<String>) -> Self {
        Self { paths }
    }
    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
          0 => panic!("invalid renderable"),
          1 => RenderableKind::Static,
          _ => RenderableKind::Animated
        }
    }
    pub fn path(&self, path_index: usize) -> String {
      self.paths[path_index % self.paths.len()].clone()
    }
}

pub enum RenderableKind {
    Static,
    Animated,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}
#[derive(PartialEq)]
pub enum BoxColour {
    Blue,
    Red,
}
impl Display for BoxColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BoxColour::Blue => "blue",
            BoxColour::Red => "red",
        })?;
        Ok(())
    }
}
#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub colour: BoxColour,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub colour: BoxColour,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}
