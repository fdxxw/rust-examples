use specs::World;

use crate::{
    components::{BoxColour, Position},
    entities::{create_box, create_box_spot, create_floor, create_player, create_wall},
};

pub fn load_map(world: &mut World, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split("\n").map(|x| x.trim()).collect();
    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(" ").collect();
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };
            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "BS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Blue);
                }
                "RS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Red);
                }
                "BB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Blue);
                }
                "RB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Red);
                }

                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
