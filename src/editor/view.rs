use std::io::Error;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
use super::terminal::Terminal;
mod buffer;

use buffer::Buffer;

#[derive(Default)]
pub struct View{
    buffer: Buffer,
}
impl View{
    pub fn render_welcome_screen()->Result<(),Error>{
        let height = Terminal::size()?.height;
        for current_row in 0..height{
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if current_row + 1 == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
    pub fn render_buffer(&self)->Result<(),Error>{
        let height = Terminal::size()?.height;
        for current_row in 0..height{
            Terminal::clear_line()?;
            if let Some(line) = self.buffer.lines.get(current_row){
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
            }else{
                Self::draw_empty_row()?;
            }
        }
        Ok(())
    }

    pub fn render(&self) -> Result<(), Error> {
        if self.buffer.is_empty(){
            Self::render_welcome_screen()?;
        }else{
            self.render_buffer()?;
        }
        Ok(())
    }
    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor --version{VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        //取整数--we allow this since we don't care if our welcome message is put _exactly_ in the middle.
        // it's allowed to be a bit to the left or right.
        #[allow(clippy::integer_division)]
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1)); //空出一个空需要打印‘~’
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width); //truncate the output to be at most as wide as the screen in case it is too long
        Terminal::print(&welcome_message)?;
        Ok(())
    }
    fn draw_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
    pub fn load(&mut self,file_name:&str){
        if let Ok(buffer) = Buffer::load(file_name){
            self.buffer = buffer;
        }
    }
}