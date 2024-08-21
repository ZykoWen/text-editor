#![warn(clippy::all, clippy::pedantic)]
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
mod editorcommand;
mod terminal;
mod view;

use editorcommand::EditorCommand;
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
use terminal::Terminal;
use view::View;
pub struct Editor {
    should_quit: bool, //增加元素，用于判断是否需要退出循环
    view: View,
}
impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        //检查程序是否接受了命令行参数作为文件名，如果是，则尝试加载文件
        if let Some(file_name) = args.get(1) {
            view.load(file_name);
        }
        Ok({
            Self {
                should_quit: false,
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
    pub fn evaluate_event(&mut self,event:Event){
        let should_process = match &event{
            Event::Key(KeyEvent{kind,..})=>kind == &KeyEventKind::Press,
            Event::Resize(_,_)=>true,
            _=>false,
        };
        if should_process{
            match EditorCommand::try_from(event){
                Ok(command)=>{
                    if matches!(command,EditorCommand::Quit){
                        self.should_quit=true;
                    }else{
                        self.view.handle_command(command);
                    }
                }
                Err(err)=>{
                    #[cfg(debug_assertions)]
                    {
                        panic!("couldn't handle the command:{err}");
                    }
                }
            }
        }
        // else{
        //     #[cfg(debug_assertions)]
        //     {
        //         panic!("Received and discarded unsupported or non-press event.");
        //     }
        // }
    }
    pub fn refresh_screen(&mut self) {
        //不知道为什么要用mut
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.get_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }
}
impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye\n\r");
        }
    }
}
