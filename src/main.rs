mod colors;
mod line_buffer;
mod status_line;
mod utils;
mod scroll_buffer;
mod editor;
mod customer;
mod logger;

use editor::Editor;


fn main() {
    match run_program() {
        Ok(_) => log::info!("Program exited successfully"),
        Err(e) => log::error!("Program failed: {}", e),
    }
}

fn run_program() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger()?;

    let mut editor = Editor::new()?;

    editor.init()?;

    editor.run()?;

    Ok(())
}


