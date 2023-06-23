mod colors;
mod line_buffer;
mod status_line;
mod utils;
mod scroll_buffer;
mod editor;
mod customer;
mod logger;

use editor::Editor;
use clap::Parser;
use directories::ProjectDirs;
use std::path::PathBuf;

fn main() {
    match run_program() {
        Ok(_) => log::info!("Program exited successfully"),
        Err(e) => log::error!("Program failed: {}", e),
    }
}

fn run_program() -> Result<(), Box<dyn std::error::Error>> {
    logger::setup_logger()?;

    let args = Args::parse();

    let file_path = args.filename
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            let proj_dirs = ProjectDirs::from("au", "popplestones", "RustyCrm").expect("Failed to get project directory");
            proj_dirs.config_dir().join("contacts.json")
        });

    let mut editor = Editor::new(file_path)?;

    editor.init()?;

    editor.run()?;

    Ok(())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to load
    #[arg(short, long)]
    filename: Option<String>,
}

