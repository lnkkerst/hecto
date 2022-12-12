use std::io::{stdout, Write};

use crossterm::{
    cursor::{self, MoveTo},
    execute,
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
    pub fn defalut() -> crossterm::Result<Self> {
        let size = terminal::size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), Clear(ClearType::All))
    }

    pub fn cursor_position(position: &Position) -> crossterm::Result<()> {
        let Position { x, y } = &position;
        let x = *x as u16;
        let y = *y as u16;
        execute!(stdout(), MoveTo(x, y))
    }

    pub fn flush() -> crossterm::Result<()> {
        stdout().flush()
    }

    pub fn cursor_hide() -> crossterm::Result<()> {
        execute!(stdout(), cursor::Hide)
    }

    pub fn cursor_show() -> crossterm::Result<()> {
        execute!(stdout(), cursor::Show)
    }

    pub fn clear_current_line() -> crossterm::Result<()> {
        execute!(stdout(), Clear(ClearType::CurrentLine))
    }
}
