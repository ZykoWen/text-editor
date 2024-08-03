use std::io::stdout;
use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{enable_raw_mode,disable_raw_mode,Clear,ClearType,size};
pub struct Terminal{}
impl Terminal{
    pub fn initialize() -> Result<(),std::io::Error>{
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0,0)?;
        Ok(())
    }
    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(),std::io::Error>{
        let mut stdout = stdout();//stdout() 是调用 stdout 函数，这个函数返回一个 Stdout 类型的值，表示标准输出流。
        execute!(stdout,Clear(ClearType::All))//execute! 是一个宏（macro），用于执行终端相关的操作。stdout 是传递给 execute! 宏的参数，表示要操作的标准输出流。
        // Clear(ClearType::All) 是传递给 execute! 宏的另一个参数，表示要执行的操作是清空屏幕。ClearType::All 是一个枚举值，表示清空整个屏幕。
    }
    pub fn move_cursor_to(x:u16,y:u16) -> Result<(),std::io::Error>{
        execute!(stdout(),MoveTo(x,y))?;
        Ok(())
    }
    pub fn size()->Result<(u16,u16),std::io::Error>{
        size()
    }
}