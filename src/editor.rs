use crate::Terminal;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::enable_raw_mode,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
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
        Self {
            should_quit: false,
            terminal: Terminal::defalut().expect("Failed to initialize terminal"),
        }
    }

    fn refresh_screen(&self) -> crossterm::Result<()> {
        Terminal::cursor_hide()?;
        Terminal::cursor_postition(0, 0)?;

        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye.\r");
        } else {
            self.draw_rows()?;
            Terminal::cursor_postition(0, 0)?;
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
            _ => {
                println!("{:?} \r", pressed_key);
            }
        }
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

    fn draw_rows(&self) -> crossterm::Result<()> {
        let height = self.terminal.size().height;
        for row in 0..height - 1 {
            Terminal::clear_current_line()?;
            if row == height / 3 {
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
