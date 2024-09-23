use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::{
    components::{Box, BoxSpot, Position},
    resources::{Gameplay, GameplayState, EventQueue}, events::Event,
};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, Gameplay>,
        Write<'a, EventQueue>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay, mut event_queue, positions, boxs, box_spots) = data;
        let boxes_by_position = (&positions, &boxs)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();
        for (_box_spot, position) in (&box_spots, &positions).join() {
            if let Some(the_box) = boxes_by_position.get(&(position.x, position.y)) {
                if the_box.colour == _box_spot.colour {
                    // continue
                } else {
                    return;
                }
            } else {
                gameplay.state = GameplayState::Playing;
                return;
            }
        }
        if gameplay.state == GameplayState::Playing {
            gameplay.state = GameplayState::Won;
            event_queue.events.push(Event::Won);
        }
    }
}
