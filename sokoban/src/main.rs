#![windows_subsystem = "windows"]
use audio::initialize_sounds;
use ggez::{
    conf,
    event::{self},
    timer, Context, GameResult,
};
use level::{initialize_levels, LevelStore};
use specs::{RunNow, World, WorldExt};
mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod level;
mod map;
mod resources;
mod systems;
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
        self.world.maintain();
        // 输入系统
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
        // 游戏状态系统
        {
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }
        // 时间资源
        {
            let mut time = self.world.write_resource::<Time>();
            time.delta += timer::delta(&_context);
        }
        // 事件系统
        {
            let mut es = EventSystem {
                context: _context,
                world: &self.world,
            };
            es.run_now(&self.world);
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

pub fn initialize_level(world: &World) {
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
    map::load_map(
        world,
        world.read_resource::<LevelStore>().level(5).to_string(),
    );
}

pub fn main() -> GameResult {
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("推箱子V1.0"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_zipfile_bytes(include_bytes!("../resources.zip").to_vec());

    let (mut context, event_loop) = context_builder.build()?;
    // Create the game state
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_levels(&mut world);
    initialize_level(&world);
    initialize_sounds(&mut world, &mut context);
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
