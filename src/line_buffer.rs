use crossterm::cursor::{MoveTo, SetCursorStyle, MoveLeft, MoveRight, MoveToColumn, SavePosition, RestorePosition};
use crossterm::style::{Colors, Print, SetColors};
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::terminal::{Clear, ClearType};
use std::io::{self, stdout, Write};

use crate::colors::ColorScheme;

#[derive(Debug)]
pub struct LineBuffer {
    buffer: String,
    caret_pos: usize,
    prompt: String,
    insert: bool,
    color_scheme: ColorScheme
}

impl LineBuffer {
    pub fn new(prompt: String, color_scheme: ColorScheme) -> Self {
        LineBuffer {
            buffer: String::new(),
            caret_pos: 0,
            prompt,
            insert: true,
            color_scheme
        }
    }

    pub fn get_string(&self) -> String {
        self.buffer.clone()
    }

    pub fn set_buffer(&mut self, buffer: String) -> io::Result<()> {
        self.buffer = buffer;
        self.caret_pos = self.buffer.len();
        self.draw()?;

        Ok(())
    }

    pub fn set_prompt(&mut self, prompt: String) -> io::Result<()> {
        log::info!("Setting prompt to {}", prompt);
        self.prompt = prompt;
        self.draw()?;

        Ok(())
    }
    pub fn clear(&mut self) -> io::Result<()> {
        self.buffer = String::new();
        self.caret_pos = 0;
        self.draw()?;

        Ok(())
    }

    pub fn draw(&self) -> io::Result<()> {
        stdout().queue(MoveTo(0, 0))?;
        self.set_colors()?;
        stdout().queue(Clear(ClearType::CurrentLine))?;
        stdout().queue(Print(&self.prompt))?;
        stdout().queue(Print(&self.buffer))?;
        self.sync_caret()?;

        Ok(())
    }
    pub fn sync_caret(&self) -> io::Result<()> {
        self.set_colors();
        stdout().queue(MoveTo((self.prompt.len()+self.caret_pos) as u16, 0))?;
        stdout().flush()?;

        Ok(())
    }
    fn set_colors(&self) -> io::Result<()> {
        stdout().queue(SetColors(Colors::new(self.color_scheme.white, self.color_scheme.dark_black)))?;
        Ok(())
    }

    pub fn add(&mut self, text: &str) -> crossterm::Result<()> {
        let mut chars: Vec<char> = self.buffer.chars().collect();

        self.set_colors()?;

        if self.insert {
            // In insert mode, add the text at the caret position
            for (i, c) in text.chars().enumerate() {
                chars.insert(self.caret_pos + i, c);
            }
            self.caret_pos += text.chars().count();

            // Print the inserted text
            stdout().queue(Print(text))?;

            // Print the rest of the buffer after the inserted text
            if self.caret_pos < chars.len() {
                let rest: String = chars[self.caret_pos..].iter().collect();
                stdout().queue(Print(rest))?;
            }
        } else {
            // In overtype mode, remove the existing text and replace it with the new text
            for (i, c) in text.chars().enumerate() {
                if self.caret_pos + i < chars.len() {
                    chars[self.caret_pos + i] = c;
                } else {
                    chars.push(c);
                }
            }
            self.caret_pos += text.chars().count();

            // Print the replaced text
            stdout().queue(Print(text))?;
        }

        self.buffer = chars.into_iter().collect();

        stdout().queue(Clear(ClearType::UntilNewLine))?;
        stdout().flush()?;
    
        self.sync_caret()?;

        Ok(())
    }

    pub fn toggle_insert(&mut self) -> io::Result<()> {
        self.insert = !self.insert;

        if self.insert {
            stdout().execute(SetCursorStyle::BlinkingBar)?;
            return Ok(());
        }

        stdout().execute(SetCursorStyle::BlinkingUnderScore)?;
        Ok(())
    }

    pub fn move_left(&mut self) -> io::Result<()> {
        if self.caret_pos > 0 {
            self.caret_pos -= 1;
            stdout().execute(MoveLeft(1))?;
        }

        Ok(())
    }
    pub fn move_right(&mut self) -> io::Result<()> {
        if self.caret_pos < self.buffer.len() {
            self.caret_pos += 1;
            stdout().execute(MoveRight(1))?;
        }

        Ok(())
    }
    pub fn move_to_start(&mut self) -> io::Result<()> {
        self.caret_pos = 0;
        stdout().execute(MoveToColumn(self.prompt.len() as u16))?;

        Ok(())
    }

    pub fn move_to_end(&mut self) -> io::Result<()> {
        self.caret_pos = self.buffer.len();

        stdout().execute(MoveToColumn((self.prompt.len()+self.caret_pos) as u16))?;

        Ok(())
    }

    pub fn delete(&mut self) -> io::Result<()> {
        // Delete from caret_pos one character from the buffer
        // and redraw the rest of the line
        if self.caret_pos < self.buffer.len() {
            self.buffer.remove(self.caret_pos);
            self.draw()?;
        }
        Ok(())
    }

    pub fn backspace(&mut self) -> io::Result<()> {
        if self.caret_pos > 0 {
            self.buffer.remove(self.caret_pos - 1);
            self.caret_pos -= 1;
            self.draw()?;
        }

        Ok(())
    }





}
