use std::io::{stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    execute, style,
    terminal::{self, Clear, ClearType},
};

use crate::Position;

#[derive(Debug)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn defalut() -> Result<Self, crossterm::ErrorKind> {
        let size = terminal::size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }

    pub fn cursor_position(position: &Position) {
        let Position { x, y } = &position;
        let x = *x as u16;
        let y = *y as u16;
        execute!(stdout(), MoveTo(x, y)).unwrap();
    }

    pub fn flush() -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn cursor_hide() {
        execute!(stdout(), cursor::Hide).unwrap();
    }

    pub fn cursor_show() {
        execute!(stdout(), cursor::Show).unwrap();
    }

    pub fn clear_current_line() {
        execute!(stdout(), Clear(ClearType::CurrentLine)).unwrap();
    }

    pub fn set_bg_color(color: style::Color) {
        execute!(stdout(), style::SetBackgroundColor(color)).unwrap();
    }

    pub fn reset_color() {
        execute!(stdout(), style::ResetColor).unwrap();
    }

    pub fn set_fg_color(color: style::Color) {
        execute!(stdout(), style::SetForegroundColor(color)).unwrap();
    }

    pub fn update_size(&mut self) -> Result<(), crossterm::ErrorKind> {
        let size = terminal::size()?;
        self.size = Size {
            width: size.0,
            height: size.1.saturating_sub(2),
        };
        Ok(())
    }
}
