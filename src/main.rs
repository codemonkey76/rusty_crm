mod contact;
mod customer;
mod note;
mod utils;
mod line_buffer;

use std::io::{self, stdout};
use crossterm::{event::{self, read, poll, Event, KeyCode, KeyModifiers}, QueueableCommand};
use crossterm::style::Print;

use utils::RawMode;
use line_buffer::LineBuffer;

fn main() -> io::Result<()> {
    let _raw_mode = RawMode::new()?;
    
    let mut buffer = LineBuffer::new("Query: ".to_string());

    buffer.draw();
    loop {
        if poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Char('q') if event.modifiers.contains(KeyModifiers::CONTROL) => { break; },
                    KeyCode::Char(c) => {
                        buffer.add(&c.to_string())?;
                    },
                    KeyCode::Insert => { buffer.toggle_insert()?; },
                    KeyCode::Left => { buffer.move_left()?; },
                    KeyCode::Right => { buffer.move_right()?; },
                    KeyCode::Home => { buffer.move_to_start()?; },
                    KeyCode::End => { buffer.move_to_end()?; },
                    _ => {}
                }
            }
        }
    }
    Ok(())
}


