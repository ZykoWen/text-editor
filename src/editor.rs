#![warn(clippy::all, clippy::pedantic)]
use crossterm::event::{read,Event,Event::Key,KeyCode::Char,KeyEvent,KeyModifiers};
use crossterm::terminal::size; //添加后两个
mod terminal;
use terminal::Terminal;

pub struct Editor{
    should_quit:bool  //增加元素，用于判断是否需要退出循环
}
impl Editor{
    //下面函数中仅仅定义了一个结构体，所以可以将其设置为const函数，
    // It enables this function to be evaluated on compile time.
    pub const fn default()->Self{
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
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit==true{
                break;
            }
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
        if self.should_quit == true{
            Terminal::clear_screen()?;
            print!("Goodbye!");
            Ok(())
        }else{
            Self::draw_rows()?;
            Terminal::move_cursor_to(0,0)?;
            Ok(())
        }
    }
    pub fn draw_rows() -> Result<(),std::io::Error>{
        let height = size()?.1;
        for current_row in 0..height{
            if current_row+1 ==height{
                print!("~")    //最后一行，输出后不换行
            }
            else{
                println!("~");  //输出后换行
            }
            // if current_row+1 < height{
            //     print!("\r\n");
            // }
        }
        Ok(())
    }
}
