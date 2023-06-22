use crossterm::cursor::{MoveTo, SetCursorStyle, MoveLeft, MoveRight, MoveToColumn};
use crossterm::style::Print;
use crossterm::{ExecutableCommand, QueueableCommand};
use std::io::{self, stdout, Write};

#[derive(Debug)]
pub struct LineBuffer {
    buffer: String,
    caret_pos: usize,
    prompt: String,
    insert: bool
}

impl LineBuffer {
    pub fn new(prompt: String) -> Self {
        LineBuffer {
            buffer: String::new(),
            caret_pos: 0,
            prompt,
            insert: true
        }
    }

    pub fn draw(&self) -> io::Result<()> {
        stdout().queue(MoveTo(0, 0))?;
        stdout().queue(Print(&self.prompt))?;
        stdout().queue(Print(&self.buffer))?;
        stdout().queue(MoveTo((self.prompt.len() +self.caret_pos) as u16, 0))?;
        stdout().flush()?;

        Ok(())
    }
    pub fn sync_caret(&self) -> io::Result<()> {
        stdout().execute(MoveTo((self.prompt.len()+self.caret_pos) as u16, 0))?;

        Ok(())
    }
    pub fn add(&mut self, text: &str) -> crossterm::Result<()> {
        let mut chars: Vec<char> = self.buffer.chars().collect();

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
        }
        stdout().execute(MoveLeft(1))?;

        Ok(())
    }
    pub fn move_right(&mut self) -> io::Result<()> {
        if self.caret_pos < self.buffer.len() {
            self.caret_pos += 1;
        }
        stdout().execute(MoveRight(1))?;

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




}
