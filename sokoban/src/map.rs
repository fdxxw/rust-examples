use specs::{LazyUpdate, World, WorldExt};

use crate::{
    components::{BoxColour, Position},
    entities::{create_box, create_box_spot, create_floor, create_player, create_wall},
    level::LevelStore,
};

pub fn load_map(world: &World, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split("\n").map(|x| x.trim()).collect();
    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(" ").collect();
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };
            let lazy = world.read_resource::<LazyUpdate>();
            let entities = world.entities();

            match *column {
                "." => create_floor(lazy.create_entity(&entities), position),
                "W" => {
                    create_floor(lazy.create_entity(&entities), position);
                    create_wall(lazy.create_entity(&entities), position);
                }
                "P" => {
                    create_floor(lazy.create_entity(&entities), position);
                    create_player(lazy.create_entity(&entities), position);
                }
                "BS" => {
                    create_floor(lazy.create_entity(&entities), position);
                    create_box_spot(lazy.create_entity(&entities), position, BoxColour::Blue);
                }
                "RS" => {
                    create_floor(lazy.create_entity(&entities), position);
                    create_box_spot(lazy.create_entity(&entities), position, BoxColour::Red);
                }
                "BB" => {
                    create_floor(lazy.create_entity(&entities), position);
                    create_box(lazy.create_entity(&entities), position, BoxColour::Blue);
                }
                "RB" => {
                    create_floor(lazy.create_entity(&entities), position);
                    create_box(lazy.create_entity(&entities), position, BoxColour::Red);
                }

                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

pub fn load_map_level(world: &World, level: u8) {
    load_map(
        world,
        world.read_resource::<LevelStore>().level(level).to_string(),
    );
}
