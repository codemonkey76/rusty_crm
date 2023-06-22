use crossterm::style::{Colors, Print, SetColors};
use crossterm::cursor::{SavePosition, RestorePosition, MoveTo, MoveToColumn};
use crossterm::terminal::{size, Clear, ClearType};
use std::io::{self, stdout, Write};
use crossterm::QueueableCommand;

use crate::colors::ColorScheme;

pub struct StatusLine {
    row: usize,
    cols: usize,
    results: usize,
    color_scheme: ColorScheme
}

impl StatusLine {
    pub fn new(color_scheme: ColorScheme) -> Result<StatusLine, io::Error> {
        let size = size()?;
        let (cols, row) = (size.0 as usize, size.1 as usize - 1);

        Ok(StatusLine {
            cols,
            row,
            results: 0,
            color_scheme
        })
    }
    pub fn draw(&self) -> io::Result<()> {
        let results_string = format!("Results: {}", self.results);
        let results_offset = results_string.len();
        stdout().queue(SetColors(Colors::new(self.color_scheme.grey, self.color_scheme.black)))?;
        stdout().queue(SavePosition)?;
        stdout().queue(MoveTo(0, self.row as u16))?;
        stdout().queue(Clear(ClearType::CurrentLine))?;
        stdout().queue(Print("Ready"))?;
        stdout().queue(MoveToColumn((self.cols - results_offset) as u16))?;
        stdout().queue(Print(results_string))?;
        stdout().queue(RestorePosition)?;
        stdout().flush()?;

        Ok(())
    }

    pub fn set_results_count(&mut self, count: usize) {
        self.results = count;
    }

}
