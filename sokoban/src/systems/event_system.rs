use std::collections::HashMap;

use ggez::Context;
use specs::{Entities, Join, ReadStorage, System, World, Write};

use crate::{
    audio::AudioStore,
    components::{Box, BoxSpot, Position},
    events::{BoxPlacedOnSpot, EntityMoved, Event},
    map::load_map_level,
    resources::{EventQueue, Gameplay, GameplayState},
};

pub struct EventSystem<'a> {
    pub context: &'a mut Context,
    pub world: &'a World,
}

impl<'a> System<'a> for EventSystem<'a> {
    type SystemData = (
        Write<'a, Gameplay>,
        Write<'a, EventQueue>,
        Write<'a, AudioStore>,
        Entities<'a>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );
    fn run(
        &mut self,
        (
            mut gameplay,
            mut event_queue,
            mut audio_store,
            entities,
            boxes,
            box_spots,
            positions,
        ): Self::SystemData,
    ) {
        let mut new_events = Vec::new();
        for event in event_queue.events.drain(..) {
            println!("New event: {:?}", event);
            match event {
                Event::PlayerHitObstacle => {
                    audio_store.play_sound(self.context, &"wall".to_string());
                }
                Event::EntityMoved(EntityMoved { id }) => {
                    if let Some(the_box) = boxes.get(entities.entity(id)) {
                        let box_spots_with_positions = (&box_spots, &positions)
                            .join()
                            .map(|t| ((t.1.x, t.1.y), t.0))
                            .collect::<HashMap<_, _>>();
                        if let Some(box_position) = positions.get(entities.entity(id)) {
                            if let Some(box_spot) =
                                box_spots_with_positions.get(&(box_position.x, box_position.y))
                            {
                                new_events.push(Event::BoxPlacedOnSpot(BoxPlacedOnSpot {
                                    is_correct_spot: box_spot.colour == the_box.colour,
                                }))
                            }
                        }
                    }
                }
                Event::BoxPlacedOnSpot(BoxPlacedOnSpot { is_correct_spot }) => {
                    let sound = if is_correct_spot {
                        "correct"
                    } else {
                        "incorrect"
                    };
                    audio_store.play_sound(self.context, &sound.to_string());
                }
                Event::Won => {
                    for entity in (&entities).join() {
                        entities.delete(entity).expect("expected delete entity");
                    }
                    gameplay.level += 1;
                    load_map_level(self.world, gameplay.level);
                    gameplay.state = GameplayState::Playing
                }
            }
        }
        event_queue.events.append(&mut new_events);
    }
}
