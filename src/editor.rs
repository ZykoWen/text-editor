#![warn(clippy::all, clippy::pedantic)]
use crossterm::event::{read, Event, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
mod view;

use view::View;
use core::cmp::min;
use std::io::Error;
use terminal::{Position, Size, Terminal};
#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}
#[derive(Default)]
pub struct Editor {
    should_quit: bool, //增加元素，用于判断是否需要退出循环
    location: Location,
    view:View,
}
impl Editor {
    pub fn run(&mut self) {
        //三个位置可能出现error：初始化、实现步骤、关闭步骤
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }
    pub fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press, //保证按一次移动键，仅仅移动一个单位
            ..
        }) = event
        {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageUp
                | KeyCode::PageDown
                | KeyCode::Home
                | KeyCode::End => {
                    self.match_code(*code)?;
                }
                _ => (),
            }
        }
        Ok(())
    }
    pub fn match_code(&mut self, code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { height, width } = Terminal::size()?;
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
        Ok(())
    }
    pub fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_caret()?;
        Terminal::move_caret_to(Position::default())?;
        if self.should_quit == true {
            Terminal::clear_screen()?;
            Terminal::print("Goodbye!")?;
        } else {
            self.view.render()?;    //初始渲染！
            Terminal::move_caret_to(Position {
                row: self.location.x,
                col: self.location.y,
            })?;
        }
        Terminal::show_caret()?;
        Terminal::execute()?;
        Ok(())
    }
}
