use audio::initialize_sounds;
use ggez::{
    conf,
    event::{self},
    Context, GameResult, timer,
};
use specs::{World, RunNow, WorldExt};
use std::path;
mod components;
mod constants;
mod entities;
mod map;
mod resources;
mod systems;
mod events;
mod audio;
use components::*;
use resources::*;
use systems::*;
// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // 输入系统
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
        // 游戏状态系统
        {
            let mut gss = GameplayStateSystem{};
            gss.run_now(&self.world);
        }
        // 时间资源
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(&_context);
        }
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSysterm { context: _context };
            rs.run_now(&self.world);
        }
        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymods: event::KeyMods,
        _repeat: bool,
    ) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.key_pressed.push(keycode);
    }
}

pub fn initialize_level(world: &mut World) {
    // const MAP: &str = "
    // N N W W W W W W
    // W W W . . . . W
    // W . . . B . . W
    // W . . . . . . W 
    // W . P . . . . W
    // W . . . . . . W
    // W . . S . . . W
    // W . . . . . . W
    // W W W W W W W W
    // ";
    const MAP:&str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    map::load_map(world, MAP.to_string());
}

pub fn main() -> GameResult {
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (mut context, event_loop) = context_builder.build()?;
    // Create the game state
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);
    initialize_sounds(&mut world, &mut context);
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
