const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
use super::terminal::{Position, Size, Terminal};
mod buffer;
use buffer::Buffer;
mod line;
mod location;
use location::Location;
use crate::editor::editorcommand::{Direction, EditorCommand};
use crate::editor::view::line::Line;
use std::cmp::min;


pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: Location,
    scroll_offset:Location,
}
impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(), //If the Result from size() returns something, it's returned. If not, the default should be used.
            location:Location::default(),
            scroll_offset:Location::default(),
        }
    }
}
impl View {
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;
        let top=self.scroll_offset.y;
        for current_row in 0..height{
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)){
                let left = self.scroll_offset.x;
                let right = left.saturating_add(width);
                Self::render_line(current_row,&line.get(left..right));
            }else if vertical_center==current_row && self.buffer.is_empty(){
                Self::render_line(current_row,&Self::build_welcome_message(width));
            }else{
                Self::render_line(current_row,"~");
            }
        }
        self.needs_redraw = false;
    }
    pub fn handle_command(&mut self,command:EditorCommand){
        match command{
            EditorCommand::Resize(size)=>self.resize(size),
            EditorCommand::Move(direction)=>self.move_text_location(&direction),
            EditorCommand::Quit=>{},
        }
    }
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }
    pub fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
    pub fn get_position(&self)->Position{
        self.location.subtract(&self.scroll_offset).into()
    }
    fn move_text_location(&mut self,direction: &Direction){
        let Location {mut x, mut y} = self.location;
        let Size{height,..} = self.size;
        match direction{
            Direction::Up=>{
                y = y.saturating_sub(1);
            }
            Direction::Down=>{
                y = y.saturating_add(1);
            }
            Direction::Left=>{
                if x>0{
                    x-=1;
                }
                else if y>0{
                    y-=1;
                    x=self.buffer.lines.get(y).map_or(0,Line::len);
                }
            }
            Direction::Right=>{
                let width = self.buffer.lines.get(y).map_or(0,Line::len);
                if x<width{
                    x+=1;
                }else{
                    y = y.saturating_add(1);
                    x=0;
                }
            }
            Direction::PageUp=>{
                y=y.saturating_sub(height).saturating_sub(1);
            }
            Direction::PageDown=>{
                y=y.saturating_add(height).saturating_sub(1);
            }
            Direction::Home=>{
                x = 0;
            }
            Direction::End=>{
                x = self.buffer.lines.get(y).map_or(0,Line::len);
            }
        }
        x = self.buffer.lines.get(y).map_or(0,|line| min(line.len(),x));
        y = min(y,self.buffer.lines.len());
        self.location = Location{x,y};
        self.scroll_location_into_view();
    }
    fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }
    fn scroll_location_into_view(&mut self){
        let Location{x,y} = self.location;
        let Size{height,width} = self.size;
        let mut offset_change = false;
        //scroll vertically
        if y<self.scroll_offset.y{
            self.scroll_offset.y = y;
            offset_change = true;
        }else if y>self.scroll_offset.y.saturating_add(height){
            self.scroll_offset.y = y.saturating_add(height);
            offset_change=true;
        }
        //scroll horizontally
        if x<self.scroll_offset.x{
            self.scroll_offset.x = x;
            offset_change = true;
        }else if x>self.scroll_offset.x.saturating_add(width){
            self.scroll_offset.x = x;
            offset_change=true;
        }
        self.needs_redraw = offset_change;
    }
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor --version{VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        let padding = width.saturating_sub(len).saturating_sub(1) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }
}
