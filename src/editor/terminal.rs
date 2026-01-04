use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::execute;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, size};
use std::io::{stdout, Write};
use core::fmt::Display;

pub struct Size {
    pub width: u16,
    pub height: u16
}

pub struct Position {
    pub x: u16,
    pub y: u16
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0, y:0})?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width, height) = size()?;
        Ok(Size {width , height})
    }
    pub fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }
    pub fn print<T: Display>(string: T) -> Result<(), std::io::Error> {
        queue!(stdout(), Print(string));
        Ok(())
    }
    pub fn execute() -> Result<(), std::io::Error> {
        stdout().flush()?;
        Ok(())
    }
}
