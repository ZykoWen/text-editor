#![warn(clippy::all, clippy::pedantic)]
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
mod view;

use core::cmp::min;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::{Position, Size, Terminal};
use view::View;
#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}
pub struct Editor {
    should_quit: bool, //增加元素，用于判断是否需要退出循环
    location: Location,
    view: View,
}
impl Editor {
    pub fn new()->Result<Self,Error>{
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args:Vec<String> = env::args().collect();
        //检查程序是否接受了命令行参数作为文件名，如果是，则尝试加载文件
        if let Some(file_name) = args.get(1){
            view.load(file_name);
        }
        Ok({
            Self{
                should_quit:false,
                location:Location::default(),
                view,
            }
        })
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                //In a Debug build, we panic if something goes wrong, in a Release build we continue evaluation.
                Ok(event) => {
                    self.evaluate_event(event);
                }
                #[cfg(debug_assertions)]
                Err(error) => {
                    panic!("Couldn't read event:{error:?}");
                }
            }
        }
    }
    pub fn evaluate_event(&mut self, event: Event){
        match event {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: KeyEventKind::Press,
                ..
            }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                }
                (
                    KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::Home
                    | KeyCode::End,
                    _,
                ) => {
                    self.move_point(code);
                }
                _ => {}
            },
            Event::Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                self.view.resize(Size { height, width });
            }
            _ => {}
        }
    }
    pub fn move_point(&mut self, code: KeyCode){
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size().unwrap_or_default();
        match code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            }
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            KeyCode::PageUp => {
                y = 0;
            }
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            }
            KeyCode::Home => {
                x = 0;
            }
            KeyCode::End => {
                x = width.saturating_sub(1);
            }
            _ => (),
        }
        self.location = Location { x, y };
    }
    pub fn refresh_screen(&mut self){
        //不知道为什么要用mut
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(Position {
            row: self.location.y,
            col: self.location.x,
        });
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
impl Drop for Editor{
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit{
            let _ = Terminal::print("Goodbye\n\r");
        }
    }
}
