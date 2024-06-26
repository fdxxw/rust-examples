use std::{collections::HashMap, time::Duration};

use ggez::{
    graphics::{self, spritebatch::SpriteBatch, DrawParam, Image},
    timer, Context,
};
use glam::Vec2;
use itertools::Itertools;
use specs::{Join, Read, ReadStorage, System};

use crate::{
    components::{Position, Renderable, RenderableKind},
    constants::TILE_WIDTH,
    resources::{Gameplay, Time},
};

pub struct RenderingSysterm<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSysterm<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, positions, renderables) = data;
        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));
        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();
        // rendering_data.sort_by_key(|&k| k.0.z);
        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.into_iter() {
            let image_path = self.get_image(renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH;
            let y: f32 = position.y as f32 * TILE_WIDTH;
            let z: u8 = position.z;
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_params);
        }
        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group.iter() {
                let image = Image::new(self.context, image_path).expect("expected image");
                let mut sprite_batch = SpriteBatch::new(image);
                for draw_param in draw_params {
                    sprite_batch.add(*draw_param);
                }
                graphics::draw(self.context, &sprite_batch, DrawParam::new())
                    .expect("expected render");
            }
        }
        self.draw_text(&format!("Level: {}", gameplay.level), 625.0, 60.0);
        self.draw_text(&format!("State: {}", gameplay.state), 625.0, 80.0);
        self.draw_text(&format!("Count: {}", gameplay.move_count), 625.0, 100.0);
        let fps = format!("FPS: {:.0}", timer::fps(self.context));
        self.draw_text(&fps, 625.0, 120.0);
        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}

impl RenderingSysterm<'_> {
    pub fn draw_text(&mut self, text_str: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_str);
        let destinaion = Vec2::new(x, y);
        let color = Some(graphics::Color::new(0.0, 0.0, 0.0, 1.));
        let dimensions = Vec2::new(0.0, 20.0);
        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destinaion),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected draw queued text");
    }
    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> String {
        let path_index = match renderable.kind() {
            RenderableKind::Static => 0,
            RenderableKind::Animated => {
                // If we have multiple, we want to select the right one based on the delta time.
                // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
                // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
                // we technically are on the next iteration of the loop (or on 0), but we will let
                // the renderable handle this logic of wrapping frames.
                ((delta.as_millis() % 1000) / 250) as usize
            }
        };
        renderable.path(path_index)
    }
}
