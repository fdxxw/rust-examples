use std::{
    fs,
    io::{self},
    process::{Command, Stdio},
};

use crossterm::event::{read, Event, KeyCode};

fn main() {
    // initialize game
    initialize();
    // load resources
    let maze = load_maze("maze01.txt").unwrap();
    let mut player = Sprite::new(&maze);
    // game loop

    loop {
        // update screen
        print_screen(&maze, &player);
        println!("");
        if let Ok(input) = read_input() {
            if input.as_str() == "ESC" {
                break;
            }
            player.move_player(input.as_str(), &maze);
        }
        // process input
        // process movement
        // process collisions
        // check game over
        // Temp: break infinite loop
        // break;
        // repeat
    }
    clean_up();
}

fn load_maze(file: &str) -> Result<Vec<String>, io::Error> {
    let contents = fs::read_to_string(file)?;
    Ok(contents.lines().map(|l| l.to_string()).collect())
}
fn print_screen(maze: &Vec<String>, player: &Sprite) {
    // Clear(ClearType::All);
    clear_screen();
    maze.iter().for_each(|line| {
        line.chars().for_each(|c| match c {
            '#' => print!("{}", c),
            _ => print!(" "),
        });
        println!("");
    });
    move_cursor(player.row, player.col);
    print!("P");
    move_cursor(maze.len() + 1, 0);
}

fn initialize() {
    let mut cb_term = Command::new("stty");
    cb_term.stdin(Stdio::inherit());
    cb_term.args(["cbreak", "-echo"]).spawn().unwrap();
}
fn clean_up() {
    let mut cooked_term = Command::new("stty");
    cooked_term.stdin(Stdio::inherit());
    cooked_term.args(["-cbreak", "echo"]).spawn().unwrap();
}
fn read_input() -> Result<String, &'static str> {
    let r = match read() {
        Ok(t) => t,
        Err(_) => return Err("read error"),
    };
    match r {
        Event::Key(key) => match key.code {
            KeyCode::Esc => return Ok(String::from("ESC")),
            KeyCode::Up => return Ok(String::from("UP")),
            KeyCode::Down => return Ok(String::from("DOWN")),
            KeyCode::Right => return Ok(String::from("RIGHT")),
            KeyCode::Left => return Ok(String::from("LEFT")),
            _ => return Err("Not"),
        },
        _ => return Err("Not"),
    }
}
fn make_move(old_row: usize, old_col: usize, dir: &str, maze: &Vec<String>) -> (usize, usize) {
    let mut new_row = old_row as i32;
    let mut new_col = old_col as i32;
    match dir {
        "UP" => {
            new_row = new_row - 1;
            if new_row < 0 {
                new_row = maze.len() as i32 - 1;
            }
        }
        "DOWN" => {
            new_row = new_row + 1;
            if new_row == maze.len() as i32 {
                new_row = 0;
            }
        }
        "RIGHT" => {
            new_col = new_col + 1;
            if new_col == maze[0].len() as i32 {
                new_col = 0;
            }
        }
        "LEFT" => {
            new_col = new_col - 1;
            if new_col < 0 {
                new_col = maze[0].len() as i32 - 1;
            }
        }
        _ => {}
    }
    (new_row as usize, new_col as usize)
}
struct Sprite {
    row: usize,
    col: usize,
}
impl Sprite {
    fn new(maze: &Vec<String>) -> Sprite {
        for (row, line) in maze.iter().enumerate() {
            for (col, c) in line.chars().enumerate() {
                if c == 'P' {
                    return Sprite { row, col };
                }
            }
        }
        Sprite { row: 0, col: 0 }
    }
    fn move_player(&mut self, dir: &str, maze: &Vec<String>) {
        let pos = make_move(self.row, self.col, dir, maze);
        self.row = pos.0;
        self.col = pos.1;
    }
}

/// MoveCursor sets the cursor position to given row and col.
///
/// Please note that ANSI is 1-based and the top left corner is (1,1), but here we are assuming
/// the user is using a zero based coordinate system where the top left corner is (0, 0)
fn move_cursor(row: usize, col: usize) {
    print!("\x1b[{};{}f", row + 1, col + 1);
}
/// ClearScreen cleans the terminal and set cursor position to the top left corner.

fn clear_screen() {
    print!("\x1b[2J");
    move_cursor(0, 0);
}
