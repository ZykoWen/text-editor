#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod editor;

use editor::Editor;
fn main() {
    // let editor = Editor::default();
    // editor.run();     我们需要改变editor中的值，所以不能将default返回的值赋值给editor,因为editor不可变
    Editor::new().unwrap().run();
}
