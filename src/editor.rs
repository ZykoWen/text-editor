#![warn(clippy::all, clippy::pedantic)]
use crossterm::event::{read,Event::Key,KeyCode::Char,KeyEvent,KeyModifiers};//添加后两个
use crossterm::terminal::{enable_raw_mode,disable_raw_mode};
pub struct Editor{
    should_quit:bool  //增加元素，用于判断是否需要退出循环
}
impl Editor{
    pub fn default()->Self{
        Editor{should_quit:false}
    }
    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        println!("Goodbye");
    }
    fn repl(&mut self)->Result<(),std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent{code,modifiers,kind,state})=read()?{
                println!("Code:{code:?},Modifiers:{modifiers:?},Kind:{kind:?},State:{state:?}");
                match code {
                    Char('q') if modifiers==KeyModifiers::CONTROL=>{
                        self.should_quit=true;  //当遇到Ctrl+q时，终止程序
                    }
                    _=>()
                }
            }
            if self.should_quit==true{
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}