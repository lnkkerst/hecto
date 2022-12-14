use std::{
    cmp, env,
    time::{Duration, Instant},
    usize,
};

use crate::{Document, Row, Terminal};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    style,
    terminal::enable_raw_mode,
};

const STATUS_FG_COLOR: style::Color = style::Color::Cyan;
const STATUS_BG_COLOR: style::Color = style::Color::DarkGrey;
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
struct StatusMessage {
    text: String,
    time: Instant,
}

impl StatusMessage {
    fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
        }
    }
}

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    status_message: StatusMessage,
}

impl Editor {
    pub fn run(&mut self) {
        enable_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }
            if self.should_quit {
                break;
            }
            if let Err(error) = self.process_event() {
                die(error);
            }
        }
    }

    pub fn default() -> Self {
        enable_raw_mode().unwrap();
        let args: Vec<String> = env::args().collect();
        let mut initial_status = String::from("HELP: Ctrl-Q = quit");
        let document = if args.len() > 1 {
            let file_name = &args[1];
            let doc = Document::open(&file_name);
            if doc.is_ok() {
                doc.unwrap()
            } else {
                initial_status = format!("ERR: Could not open file: {}", file_name);
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::defalut().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
            status_message: StatusMessage::from(initial_status),
        }
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Terminal::cursor_hide()?;
        Terminal::cursor_position(&Position::default())?;

        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye.\r");
        } else {
            self.draw_rows()?;
            self.draw_status_bar()?;
            self.draw_message_bar()?;
            let Position { mut x, mut y } = self.cursor_position;
            x = x.saturating_sub(self.offset.x);
            x = if let Some(row) = self.document.row(y) {
                cmp::min(x, row.len().saturating_sub(self.offset.x))
            } else {
                0
            };
            y = y.saturating_sub(self.offset.y);
            Terminal::cursor_position(&Position { x, y })?;
        }
        Terminal::cursor_show()?;
        Terminal::flush()
    }

    fn process_event(&mut self) -> crossterm::Result<()> {
        let event = event::read()?;

        if let Event::Key(pressed_key) = event {
            self.process_keypress(pressed_key);
        }

        Ok(())
    }

    fn process_keypress(&mut self, pressed_key: KeyEvent) {
        match (pressed_key.modifiers, pressed_key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => {
                self.should_quit = true;
            }
            (
                _,
                KeyCode::Up
                | KeyCode::Left
                | KeyCode::Down
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::End
                | KeyCode::Home,
            ) => {
                self.move_cursor(pressed_key.code);
            }
            _ => {
                println!("{:?} \r", pressed_key);
            }
        }
        self.scroll();
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

    fn move_cursor(&mut self, key: KeyCode) {
        let terminal_height = self.terminal.size().height as usize;
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.row(y) {
            row.len()
        } else {
            0
        };
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
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
                }
            }
            KeyCode::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height as usize
                } else {
                    height
                }
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

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Hecto editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message);
    }

    pub fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) -> crossterm::Result<()> {
        let height = self.terminal.size().height;
        for terminal_row in 0..height {
            Terminal::clear_current_line()?;
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
        Ok(())
    }

    fn draw_status_bar(&self) -> crossterm::Result<()> {
        let mut status;
        let width = self.terminal.size().width as usize;
        let mut file_name = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        status = format!("{} - {} lines", file_name, self.document.len());
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
        Terminal::set_bg_color(STATUS_BG_COLOR)?;
        Terminal::set_fg_color(STATUS_FG_COLOR)?;
        println!("{}\r", status);
        Terminal::reset_color()?;
        Ok(())
    }

    fn draw_message_bar(&self) -> crossterm::Result<()> {
        Terminal::clear_current_line()?;
        let message = &self.status_message;
        if Instant::now() - message.time < Duration::new(5, 0) {
            let mut text = message.text.clone();
            text.truncate(self.terminal.size().width as usize);
            print!("{}", text);
        }
        Ok(())
    }
}

fn die(error: crossterm::ErrorKind) {
    Terminal::clear_screen().unwrap();
    panic!("{}", error);
}
