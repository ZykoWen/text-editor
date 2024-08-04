#![warn(clippy::all, clippy::pedantic)]

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;
use terminal::{Terminal,Position};

pub struct Editor{
    should_quit:bool  //增加元素，用于判断是否需要退出循环
}
impl Editor{
    pub const fn default()->Self{
        //下面函数中仅仅定义了一个结构体，所以可以将其设置为const函数，
        // It enables this function to be evaluated on compile time.
        // Editor{should_quit:false}
        Self{should_quit:false}//不用再次重复结构体名称，同时可以避免之后改变结构体名字，此处还使用原名称
    }
    pub fn run(&mut self) {
        //三个位置可能出现error：初始化、实现步骤、关闭步骤
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    pub fn repl(&mut self)->Result<(),std::io::Error> {
        loop {
            // let event = read()?;
            // self.evaluate_event(&event);
            // self.refresh_screen()?;
            // if self.should_quit==true{
            //     break;
            // }
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    pub fn evaluate_event(&mut self,event:&Event){
        if let Key(KeyEvent{code,modifiers,..})=event{
            match code{
                Char('q') if *modifiers == KeyModifiers::CONTROL =>{
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    pub fn refresh_screen(&self) -> Result<(),std::io::Error>{
        Terminal::hide_cursor()?;
        if self.should_quit == true{
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!")?;
        }else{
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x:0,y:0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }
    pub fn draw_rows() -> Result<(),std::io::Error>{
        let height = Terminal::size()?.height;
        for current_row in 0..height{
            Terminal::clear_line()?;//先清理当前行，再写波浪线
            Terminal::print("~")?;
            if current_row+1 < height{
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}
