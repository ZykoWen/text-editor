use super::terminal::Size;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}
pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
}
impl TryFrom<Event> for EditorCommand {
    type Error = String; //String type is easily to log
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up,_)=>Ok(Self::Move(Direction::Up)),
                (KeyCode::Down,_)=>Ok(Self::Move(Direction::Down)),
                (KeyCode::Left,_)=>Ok(Self::Move(Direction::Left)),
                (KeyCode::Right,_)=>Ok(Self::Move(Direction::Right)),
                (KeyCode::Home,_)=>Ok(Self::Move(Direction::Home)),
                (KeyCode::End,_)=>Ok(Self::Move(Direction::End)),
                (KeyCode::PageUp,_)=>Ok(Self::Move(Direction::PageUp)),
                (KeyCode::PageDown,_)=>Ok(Self::Move(Direction::PageDown)),
                _=>Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(width_u16,height_u16)=>{
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                Ok(Self::Resize(Size{height,width}))
            }
            _=>Err(format!("Event not supported:{event:?}")),
        }
    }
}
