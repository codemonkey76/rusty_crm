mod colors;
mod line_buffer;
mod status_line;
mod utils;
mod scroll_buffer;
mod editor;
mod customer;
use editor::Editor;

fn main() {
    let mut editor = Editor::new().expect("Error creating new editor instance");

    editor.init().expect("Some error initializing");

    editor.run().expect("An error occurred running editor");
}

