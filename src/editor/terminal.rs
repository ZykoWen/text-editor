use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::{queue, Command};
use std::fmt::Display;
use std::io::stdout;
use std::io::Error;
use std::io::Write;

pub struct Terminal; //无字段的结构体
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
/// Represents the Terminal.
/// Edge Case for platforms where `usize` < `u16`:
/// Regardless of the actual size of the Terminal, this representation
/// only spans over at most `usize::MAX` or `u16::size` rows/columns, whichever is smaller.
/// Each size returned truncates to min(`usize::MAX`, `u16::MAX`)
/// And should you attempt to set the cursor out of these bounds, it will also be truncated.
impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(()) //Clear(ClearType::All) 是传递给 execute! 宏的另一个参数，表示要执行的操作是清空屏幕。ClearType::All 是一个枚举值，表示清空整个屏幕。
    }
    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    ///Moves the cursor to eht given Position
    /// #Arguments
    /// * 'Position' - the 'Position' to move the cursor to.Will be truncated to 'u16::MAX' if bigger
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.x as u16, position.y as u16))?;
        Ok(())
    }
    ///Returns the current size of the terminal
    /// Edge Case for systems with `usize` < `u16`:
    /// * A `Size` representing the terminal size. Any coordinate `z` truncated to `usize` if `usize` < `z` < `u16`
    pub fn size() -> Result<Size, Error> {
        #[allow(clippy::as_conversions)]
        Ok(Size {
            height: size()?.1 as usize,
            width: size()?.0 as usize,
        })
    }
    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    pub fn print<T: Display>(string: T) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }
    pub fn execute() -> Result<(), Error> {
        //确保所有的输出都写入到输出设备
        stdout().flush()?;
        Ok(())
    }
    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        //receive something that implements the Command trait - which is what we can pass to queue!.
        queue!(stdout(), command)?;
        Ok(())
    }
}
