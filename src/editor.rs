use crate::colors::ColorScheme;
use crate::line_buffer::LineBuffer;
use crate::scroll_buffer::ScrollBuffer;
use crate::status_line::StatusLine;
use crate::utils::RawMode;
use crossterm::event::{read, poll, Event, KeyCode, KeyModifiers};
use std::io;
use std::path::PathBuf;


pub struct Editor {
    pub file_path: PathBuf,
    pub line_buffer: LineBuffer,     // The line buffer
    pub scroll_buffer: ScrollBuffer, // The scroll buffer
    pub status_line: StatusLine,     // The status line
    pub color_scheme: ColorScheme,   // The color scheme
    _raw_mode: RawMode,              // The raw mode
}

impl Editor {
    pub fn new(file_path: PathBuf) -> Result<Editor, std::io::Error> {
        log::info!("Initializing editor");
        let color_scheme = ColorScheme::new();
        let line_buffer = LineBuffer::new("Query: ".to_string(), color_scheme.clone());
        let scroll_buffer = ScrollBuffer::new(color_scheme.clone())?;
        let status_line = StatusLine::new(color_scheme.clone())?;
        log::info!("Editor initialized");
        log::info!("Starting RawMode");

        let _raw_mode = RawMode::new()?;

        Ok(Editor {
            file_path,
            line_buffer,
            scroll_buffer,
            status_line,
            color_scheme,
            _raw_mode
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            if poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(event) = read()? {
                    match event.code {
                        KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => {
                            log::info!("Exiting editor loop, received CTRL+Q");
                            break;
                        },
                        KeyCode::Char('s') if event.modifiers.contains(KeyModifiers::CONTROL) => { self.save()?; },
                        KeyCode::Char('a') if event.modifiers.contains(KeyModifiers::CONTROL) => { self.add_customer()?; },
                        KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => { self.delete_customers()?; },
                        KeyCode::Char(c) => { self.add_key(c)?; },
                        KeyCode::Insert => { self.toggle_insert()?; },
                        KeyCode::Left => { self.move_left()?; },
                        KeyCode::Right => { self.move_right()?; },
                        KeyCode::Up => { self.move_up()?; },
                        KeyCode::Down => { self.move_down()?; },
                        KeyCode::Home => { self.move_to_start()?; },
                        KeyCode::End => { self.move_to_end()?; },
                        KeyCode::Delete => { self.delete()?; },
                        KeyCode::Backspace => { self.backspace()?; },
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.scroll_buffer.load_customers(self.file_path.clone());

        self.scroll_buffer.set_filter(self.line_buffer.get_string())?;
        self.line_buffer.draw()?;
        self.scroll_buffer.draw()?;
        self.status_line.set_results_count(self.scroll_buffer.get_results_count())?;
        self.line_buffer.sync_caret()?;

        Ok(())
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.scroll_buffer.save_customers(self.file_path.clone())?;
        self.status_line.set_message("Saved".to_string())?;

        Ok(())
    }

    pub fn add_key(&mut self, c: char) -> io::Result<()> {
        self.line_buffer.add(&c.to_string())?;
        log::info!("Setting filter on scroll_buffer: {}", self.line_buffer.get_string());
        self.scroll_buffer.set_filter(self.line_buffer.get_string())?;
        self.status_line.set_results_count(self.scroll_buffer.get_results_count())?;
        self.line_buffer.sync_caret()?;

        Ok(())
    }

    pub fn add_customer(&mut self) -> io::Result<()> {
        self.status_line.set_message("Add customer".to_string())?;
        Ok(())
    }

    pub fn delete_customers(&mut self) -> io::Result<()> {
        self.status_line.set_message("Deleting customers".to_string())?;
        Ok(())
    }

    pub fn toggle_insert(&mut self) -> io::Result<()> {
        self.line_buffer.toggle_insert()?;

        Ok(())
    }

    pub fn move_left(&mut self) -> io::Result<()> {
        self.line_buffer.move_left()?;

        Ok(())
    }

    pub fn move_right(&mut self) -> io::Result<()> {
        self.line_buffer.move_right()?;

        Ok(())
    }

    pub fn move_to_start(&mut self) -> io::Result<()> {
        self.line_buffer.move_to_start()?;

        Ok(())
    }

    pub fn move_to_end(&mut self) -> io::Result<()> {
        self.line_buffer.move_to_end()?;

        Ok(())
    }

    pub fn delete(&mut self) -> io::Result<()> {
        self.line_buffer.delete()?;
        self.scroll_buffer.set_filter(self.line_buffer.get_string())?;
        self.status_line.set_results_count(self.scroll_buffer.get_results_count())?;
        self.line_buffer.sync_caret()?;


        Ok(())
    }

    pub fn backspace(&mut self) -> io::Result<()> {
        self.line_buffer.backspace()?;
        self.scroll_buffer.set_filter(self.line_buffer.get_string())?;
        self.status_line.set_results_count(self.scroll_buffer.get_results_count())?;
        self.line_buffer.sync_caret()?;

        Ok(())
    }

    pub fn move_up(&mut self) -> io::Result<()> {
        self.scroll_buffer.move_up()?;
        self.line_buffer.sync_caret()?;

        Ok(())
    }

    pub fn move_down(&mut self) -> io::Result<()> {
        self.scroll_buffer.move_down()?;
        self.line_buffer.sync_caret()?;

        Ok(())
    }
}
