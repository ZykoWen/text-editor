use std::io::stdout;
use std::io::Write;
use std::io::Error;
use crossterm::queue;
use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode,disable_raw_mode,Clear,ClearType,size};

pub struct Terminal; //无字段的结构体
pub struct Size{
    pub height:u16,
    pub width:u16,
}
#[derive(Copy, Clone)]
pub struct Position{
    pub x:u16,
    pub y:u16,
}
impl Terminal{
    pub fn initialize() -> Result<(),Error>{
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0,y:0})?;
        Self::execute()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(),Error>{
        queue!(stdout(),Clear(ClearType::All))?;
        Ok(()) //Clear(ClearType::All) 是传递给 execute! 宏的另一个参数，表示要执行的操作是清空屏幕。ClearType::All 是一个枚举值，表示清空整个屏幕。
    }
    pub fn clear_line() -> Result<(),Error>{
        queue!(stdout(),Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn move_cursor_to(position:Position) -> Result<(),Error>{
        queue!(stdout(),MoveTo(position.x,position.y))?;
        Ok(())
    }
    pub fn size()->Result<Size,Error>{
        Ok(Size{
            height:size()?.1,
            width:size()?.0,
        })
    }
    pub fn hide_cursor() -> Result<(),Error>{
        queue!(stdout(),Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(),Error>{
        queue!(stdout(),Show)?;
        Ok(())
    }
    pub fn print(content:&str)->Result<(),Error>{
        queue!(stdout(),Print(content))?;
        Ok(())
    }
    pub fn execute() -> Result<(),Error>{
        //确保所有的输出都写入到输出设备
        stdout().flush()?;
        Ok(())
    }
}