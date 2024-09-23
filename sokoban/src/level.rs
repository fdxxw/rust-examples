use specs::{World, WorldExt};

#[derive(Default)]
pub struct LevelStore {
    pub levels: Vec<String>,
}

impl LevelStore {
    pub fn level(&self, level: u8) -> &String {
        &self.levels[level as usize]
    }
}

pub fn initialize_levels(world: &mut World) {
    let mut level_store = world.write_resource::<LevelStore>();
    let original_string =
        String::from_utf8_lossy(include_bytes!("../resources/levels/original.lvl"));
    let mut level = "".to_string();
    original_string.lines().for_each(|line| {
        // 新行
        if line.trim().is_empty() {
          if !level.is_empty() {
            level_store.levels.push(level.clone());
          }
          level = "".to_string();
        } else {
            for c in line.chars() {
                match c {
                    '#' => level.push('W'),
                    '@' => level.push('P'),
                    '$' => level.push_str("BB"),
                    '.' => level.push_str("BS"),
                    _ => level.push('.'),
                }
                level.push(' ');
            }
            level.push('\n');
        }
    });
}
