use crossterm::cursor::{MoveTo, MoveRight, Hide, Show};
use crossterm::execute;
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType, size};
use std::io::{stdout, Write};
use core::fmt::Display;

pub struct Size {
    pub width: usize,
    pub height: usize
}

pub struct Position {
    pub col: usize,
    pub row: usize
}

pub struct Terminal;

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{col:0, row:0})?;
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
    pub fn clear_line() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }
    pub fn move_cursor(row:u16, col: u16) -> Result<(), std::io::Error>{
        queue!(stdout(), MoveTo(row, col));
        Ok(())
    }
    pub fn size() -> Result<Size, std::io::Error> {
        let (width_u16, height_u16) = size()?;
        let height = height_u16 as usize;
        let width = width_u16 as usize;
        Ok(Size {width, height})
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
