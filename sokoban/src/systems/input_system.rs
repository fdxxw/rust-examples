use ggez::event::KeyCode;
use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use crate::{
    components::{Immovable, Movable, Player, Position},
    constants::{MAP_HEIGHT, MAP_WIDTH},
    events::{EntityMoved, Event},
    resources::{EventQueue, Gameplay, InputQueue},
};
use std::collections::HashMap;
pub struct InputSystem {}
impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, Gameplay>,
        Write<'a, EventQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut input_queue,
            mut gameplay,
            mut event_queue,
            entities,
            mut positions,
            players,
            movables,
            immovables,
        ) = data;
        let mut to_move = Vec::new();
        for (position, _player) in (&positions, &players).join() {
            if let Some(key) = input_queue.key_pressed.pop() {
                // get all the movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };
                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };
                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => match immov.get(&pos) {
                            Some(_id) => {
                                to_move.clear();
                                event_queue.events.push(Event::PlayerHitObstacle {});
                                // 碰到障碍物事件
                            }
                            None => break,
                        },
                    }
                }
            }
        }
        if to_move.len() > 0 {
            gameplay.move_count += 1; // 增加移动次数
        }
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
            event_queue
                .events
                .push(Event::EntityMoved(EntityMoved { id })); // 实体移动事件
        }
    }
}
