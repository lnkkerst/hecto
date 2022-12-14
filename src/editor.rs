use std::env;

use crate::{Document, Row, Terminal};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::enable_raw_mode,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
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
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            terminal: Terminal::defalut().expect("Failed to initialize terminal"),
            cursor_position: Position::default(),
            document,
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
            Terminal::cursor_position(&self.cursor_position)?;
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
    }

    fn move_cursor(&mut self, key: KeyCode) {
        let Position { mut x, mut y } = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            KeyCode::PageUp => y = 0,
            KeyCode::PageDown => y = height,
            KeyCode::Home => x = 0,
            KeyCode::End => x = width,
            _ => (),
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
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row);
    }

    fn draw_rows(&self) -> crossterm::Result<()> {
        let height = self.terminal.size().height;
        for terminal_row in 0..height - 1 {
            Terminal::clear_current_line()?;
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if self.document.is_empty() && terminal_row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
        Ok(())
    }
}

fn die(error: crossterm::ErrorKind) {
    Terminal::clear_screen().unwrap();
    panic!("{}", error);
}
