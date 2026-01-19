use super::terminal::{Terminal, Size};

const NAME : &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct View{
}

impl View {
    pub fn render()->Result<(),std::io::Error>{
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
            if current_row.saturating_add(1) < height {
                print!("\r\n");
            }
        }
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