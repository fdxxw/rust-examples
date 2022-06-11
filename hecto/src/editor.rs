use std::env;
use std::time::Duration;
use std::time::Instant;

use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;
use crossterm::style::Color;

use crate::Document;
use crate::Row;
use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const STATUS_BG_COLOR: Color = Color::Rgb {
    r: 239,
    g: 239,
    b: 239,
};
const STATUS_FG_COLOR: Color = Color::Rgb {
    r: 63,
    g: 63,
    b: 63,
};
const QUIT_TIMES: u8 = 3;
#[derive(PartialEq, Copy, Clone)]
pub enum SearchDirection {
    Forward,
    Backward,
}
#[derive(Default, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
struct StatusMessage {
    text: String,
    time: Instant,
}
impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
    quit_times: u8,
}
impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-S = save | Ctrl-Q | Esc = quit");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(file_name);
            if let Ok(doc) = doc {
                doc
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            offset: Position::default(),
            document,
            status_message: StatusMessage::from(initial_status),
            quit_times: QUIT_TIMES,
        }
    }
    pub fn run(&mut self) {
        // for key in io::stdin().keys() {
        //     match key {
        //         Ok(key) => match key {
        //             Key::Char(c) => {
        //                 if c.is_control() {
        //                     println!("{:?} \r", c as u8);
        //                 } else {
        //                     println!("{:?} ({})\r", c as u8, c);
        //                 }
        //             }
        //             Key::Ctrl('q') => {
        //                 break;
        //             }
        //             _ => println!("{:?}\r", key),
        //         },
        //         Err(err) => die(err),
        //     }
        // }
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if let Err(error) = self.process_keypress() {
                die(error)
            }
            if self.should_quit {
                break;
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        // print!("\x1b[2J");
        // print!("{}", termion::clear::All);
        // print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        Terminal::cursor_hide();
        // Terminal::clear_screen();
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye. \r");
        } else {
            self.draw_rows();
            self.draw_status_bar();
            self.draw_message_bar();
            // Terminal::cursor_position(&self.cursor_position);
            Terminal::cursor_position(&Position {
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let event = Terminal::read_key()?;
        if let Event::Key(pressed_key) = event {
            match (pressed_key.modifiers, pressed_key.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('q')) | (_, KeyCode::Esc) => {
                    if self.quit_times > 0 && self.document.is_dirty() {
                        self.status_message = StatusMessage::from(format!("WARNING! File has unsaved changes. Press Ctrl-Q | Esc {} more times to quit.", self.quit_times));
                        self.quit_times -= 1;
                        return Ok(());
                    }
                    self.should_quit = true
                }
                (KeyModifiers::CONTROL, KeyCode::Char('s')) => {
                    self.save();
                }
                (KeyModifiers::CONTROL, KeyCode::Char('f')) => self.search(),
                (_, KeyCode::Char(c)) => {
                    self.document.insert(&self.cursor_position, c);
                    self.move_cursor(KeyCode::Right);
                }
                (_, KeyCode::Enter) => {
                    self.document.insert(&self.cursor_position, '\n');
                    self.move_cursor(KeyCode::Right);
                }
                (_, KeyCode::Delete) => self.document.delete(&self.cursor_position),
                (_, KeyCode::Backspace) => {
                    if self.cursor_position.x > 0 || self.cursor_position.y > 0 {
                        self.move_cursor(KeyCode::Left);
                        self.document.delete(&self.cursor_position);
                    }
                }
                (_, KeyCode::Up)
                | (_, KeyCode::Down)
                | (_, KeyCode::Left)
                | (_, KeyCode::Right)
                | (_, KeyCode::PageUp)
                | (_, KeyCode::PageDown)
                | (_, KeyCode::Home)
                | (_, KeyCode::End) => self.move_cursor(pressed_key.code),
                _ => (),
            }
        }
        self.scroll();
        if self.quit_times < QUIT_TIMES {
            self.quit_times = QUIT_TIMES;
            self.status_message = StatusMessage::from(String::new());
        }
        Ok(())
    }
    fn draw_welcode_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                // let welcome_message = format!("Hecto editor -- version {}\r", VERSION);
                // let width =
                //     std::cmp::min(self.terminal.size().width as usize, welcome_message.len());
                // println!("{}\r", &welcome_message[..width]);
                self.draw_welcode_message();
            } else {
                println!("~\r");
            }
        }
    }
    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }
    fn move_cursor(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;
        // let size = self.terminal.size();
        // let height = size.height.saturating_sub(1) as usize;
        let height = self.document.len();
        // let width = size.width.saturating_sub(1) as usize;
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            }
            KeyCode::Left => {
                if x > 0 {
                    x -= 1;
                } else if y > 0 {
                    y -= 1;
                    if let Some(row) = self.document.row(y) {
                        x = row.len();
                    } else {
                        x = 0;
                    }
                }
            }
            KeyCode::Right => {
                if x < width {
                    x += 1;
                } else if y < height {
                    y += 1;
                    x = 0;
                }
            }
            KeyCode::PageUp => {
                y = if y > terminal_height {
                    y - terminal_height
                } else {
                    0
                };
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height as usize
                } else {
                    0
                };
            }
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        if x > width {
            x = width;
        }
        self.cursor_position = Position { x, y }
    }
    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    fn draw_status_bar(&self) {
        // let spaces = " ".repeat(self.terminal.size().width as usize);

        let width = self.terminal.size().width as usize;
        let modified_indicator = if self.document.is_dirty() {
            " (modified)"
        } else {
            ""
        };
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        let mut status = format!(
            "{} - {} lines{}",
            file_name,
            self.document.len(),
            modified_indicator
        );
        // if width > status.len() {
        //     status.push_str(&" ".repeat(width - status.len()));
        // }
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len() + line_indicator.len();
        if width > len {
            status.push_str(&" ".repeat(width - len));
        }
        status = format!("{}{}", status, line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r", status);
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }
    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
    }
    fn prompt<C>(&mut self, prompt: &str, mut callback: C) -> Result<Option<String>, std::io::Error>
    where
        C: FnMut(&mut Self, Event, &String),
    {
        let mut result = String::new();
        loop {
            self.status_message = StatusMessage::from(format!("{}{}", prompt, result));
            self.refresh_screen()?;
            let event = Terminal::read_key()?;
            if let Event::Key(pressed_key) = event {
                match pressed_key.code {
                    KeyCode::Backspace => {
                        if !result.is_empty() {
                            result.truncate(result.len() - 1);
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    KeyCode::Esc => {
                        result.truncate(0);
                        break;
                    }
                    KeyCode::Char(c) => {
                        if !c.is_control() {
                            result.push(c);
                        }
                    }
                    _ => (),
                }
                callback(self, event, &result);
            }
        }
        self.status_message = StatusMessage::from(String::new());
        if result.is_empty() {
            return Ok(None);
        }
        Ok(Some(result))
    }
    fn save(&mut self) {
        if self.document.file_name.is_none() {
            let new_name = self.prompt("Save as: ", |_, _, _| {}).unwrap_or(None);
            if new_name.is_none() {
                self.status_message = StatusMessage::from("Save aborted.".to_string());
                return;
            }
            self.document.file_name = new_name;
        }
        if self.document.save().is_ok() {
            self.status_message = StatusMessage::from("File saved successfully.".to_string());
        } else {
            self.status_message = StatusMessage::from("Error writting file!".to_string());
        }
    }
    fn search(&mut self) {
        let old_position = self.cursor_position.clone();
        let mut direction = SearchDirection::Forward;
        let query = self
            .prompt(
                "Search (ESC to cancel, Arraws to navigate): ",
                |editor, event, query| {
                    let mut moved = false;
                    if let Event::Key(event) = event {
                        match event.code {
                            KeyCode::Right | KeyCode::Down => {
                                direction = SearchDirection::Forward;
                                editor.move_cursor(KeyCode::Right);
                                moved = true;
                            }
                            KeyCode::Up | KeyCode::Left => direction = SearchDirection::Backward,
                            _ => direction = SearchDirection::Forward,
                        }
                    }
                    if let Some(position) =
                        editor
                            .document
                            .find(&query, &editor.cursor_position, direction)
                    {
                        editor.cursor_position = position;
                        editor.scroll();
                    } else if moved {
                        editor.move_cursor(KeyCode::Left);
                    }
                },
            )
            .unwrap_or(None);
        if query.is_none() {
            self.cursor_position = old_position;
            self.scroll();
        }
    }
}

fn die(err: std::io::Error) {
    // print!("{}", termion::clear::All);
    Terminal::clear_screen();
    panic!("{}", err);
}
