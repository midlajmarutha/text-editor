use crossterm::event::{Event::Key, read, Event, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;

use terminal::{Terminal, Position, Size};

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit : bool
}

impl Editor {
    pub fn default() -> Self{
        Editor {
            should_quit : false
        }
    }

    pub fn run(&mut self){
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let Size {height, ..} = Terminal::size()?;
        for current_row in 0..height {
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            }
            else {
                print!("~");
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

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _=>(),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(),std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Goodbye!!\r");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x:0, y:0})?;
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
