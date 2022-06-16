use specs::{world::LazyBuilder, Builder};

use crate::components::{
    Box, BoxColour, BoxSpot, Immovable, Movable, Player, Position, Renderable, Wall,
};

pub fn create_wall(builder: LazyBuilder, position: Position) {
    builder
        .with(Position { z: 10, ..position })
        .with(Renderable::new_static(String::from("/images/wall.png")))
        .with(Wall {})
        .with(Immovable)
        .build();
}
pub fn create_floor(builder: LazyBuilder, position: Position) {
    builder
        .with(Position { z: 5, ..position })
        .with(Renderable::new_static(String::from("/images/floor.png")))
        .build();
}
pub fn create_box(builder: LazyBuilder, position: Position, colour: BoxColour) {
    builder
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            format!("/images/box_{}_1.png", colour),
            format!("/images/box_{}_2.png", colour),
        ]))
        .with(Box { colour })
        .with(Movable)
        .build();
}
pub fn create_box_spot(builder: LazyBuilder, position: Position, colour: BoxColour) {
    builder
        .with(Position { z: 9, ..position })
        .with(Renderable::new_static(format!(
            "/images/box_spot_{}.png",
            colour
        )))
        .with(BoxSpot { colour })
        .build();
}
pub fn create_player(builder: LazyBuilder, position: Position) {
    builder
        .with(Position { z: 10, ..position })
        .with(Renderable::new_animated(vec![
            "/images/player_1.png".to_string(),
            "/images/player_2.png".to_string(),
            "/images/player_3.png".to_string(),
        ]))
        .with(Player {})
        .with(Movable)
        .build();
}
