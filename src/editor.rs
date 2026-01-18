use crossterm::event::{Event::Key, KeyCode::Char, read, Event, KeyCode, KeyEvent, KeyModifiers};
use core::cmp::min;
mod terminal;

use terminal::{Terminal, Position, Size};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
#[derive(Copy,Clone,Default)]
struct Location {
    x: usize,
    y: usize
}

#[derive(Default)]
pub struct Editor {
    should_quit : bool,
    location: Location
}

impl Editor {
    pub fn run(&mut self){
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows(&mut self) -> Result<(), std::io::Error> {
        let Size {height, width} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_line()?;
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            }
            else {
                // if self.location.x == 0 {
                    print!("~");
                //}k
            }
            if current_row + 1 < height {
                print!("\r\n");
            }
        }
        Ok(())
    }

    fn repl(&mut self) -> Result<(),std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn move_point(&mut self, key_code: KeyCode) -> Result<(), std::io::Error>{
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1)) 
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _=>(),
        }
        self.location = Location {x, y};
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) -> Result<(),std::io::Error>{
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up |
                KeyCode::Down |
                KeyCode::Right |
                KeyCode::Left |
                KeyCode::PageUp |
                KeyCode::PageDown |
                KeyCode::Home|
                KeyCode::End => {
                    self.move_point(*code)?;
                }
                //Right => {
                //    Terminal::move_cursor_right(1);
                //}
                _=>(),
            }
        }
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(),std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye!!\r");
        } else {
            self.draw_rows()?;
            Terminal::move_cursor_to(Position{col:self.location.x, row:self.location.y})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    fn draw_welcome_message() -> Result<(), std::io::Error>{
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(welcome_message)?;
        Ok(())
    }
}
