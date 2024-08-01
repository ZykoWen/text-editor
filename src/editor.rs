#![warn(clippy::all, clippy::pedantic)]
use crossterm::event::{read,Event,Event::Key,KeyCode::Char,KeyEvent,KeyModifiers};//添加后两个
use crossterm::execute;
use std::io::stdout;
use crossterm::terminal::{enable_raw_mode,disable_raw_mode,Clear,ClearType};
pub struct Editor{
    should_quit:bool  //增加元素，用于判断是否需要退出循环
}
impl Editor{
    pub fn default()->Self{
        Editor{should_quit:false}
    }
    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }
    fn initialize() -> Result<(),std::io::Error>{
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(),std::io::Error>{
        let mut stdout = stdout();//stdout() 是调用 stdout 函数，这个函数返回一个 Stdout 类型的值，表示标准输出流。
        execute!(stdout,Clear(ClearType::All))//execute! 是一个宏（macro），用于执行终端相关的操作。stdout 是传递给 execute! 宏的参数，表示要操作的标准输出流。
        // Clear(ClearType::All) 是传递给 execute! 宏的另一个参数，表示要执行的操作是清空屏幕。ClearType::All 是一个枚举值，表示清空整个屏幕。

    }
    fn repl(&mut self)->Result<(),std::io::Error> {
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
    fn evaluate_event(&mut self,event:&Event){
        if let Key(KeyEvent{code,modifiers,..})=event{
            match code{
                Char('q') if *modifiers == KeyModifiers::CONTROL =>{
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn refresh_screen(&self)->Result<(),std::io::Error>{
        if self.should_quit{
            Self::clear_screen()?;
            print!("Googbye.\r\n");
        }
        Ok(())
    }
}