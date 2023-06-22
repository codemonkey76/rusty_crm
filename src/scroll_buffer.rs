use crate::colors::ColorScheme;
use crate::customer::Customer;
use std::io::{self, Write, stdout};
use crossterm::cursor::{SavePosition, RestorePosition, MoveTo, MoveToNextLine};
use crossterm::style::{Print, SetColors, Colors };
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::QueueableCommand;

pub struct ScrollBuffer {
    buffer: Vec<Customer>,
    filter: String,
    filtered: Vec<usize>,
    scroll_pos: usize,
    rows: usize,
    cols: usize,
    color_scheme: ColorScheme
}


impl ScrollBuffer {
    pub fn new(color_scheme: ColorScheme) -> Result<Self, io::Error> {
        let size = size()?;
        let (cols, rows) = (size.0 as usize, size.1 as usize - 2);

        Ok(ScrollBuffer {
            buffer: Vec::new(),
            filtered: Vec::new(),
            filter: String::new(),
            scroll_pos: 0,
            cols,
            rows,
            color_scheme
        })
    }

    pub fn load_customers(&mut self) {
        self.buffer = Customer::generate(100);
    }

    pub fn set_filter(&mut self, filter: String) -> io::Result<()> {
        self.filter = filter;
        let filter_clone = self.filter.clone();
        self.filtered = self.buffer.iter().enumerate().filter(|(_, c)| c.name.contains(&filter_clone)).map(|(i, _)| i).collect();
        self.scroll_pos = 0;
        self.draw()?;

        Ok(())
    }

    pub fn draw(&self) -> io::Result<()> {
        stdout().queue(SavePosition)?;
        stdout().queue(MoveTo(0, 1))?;

        // Calculate the start and end index for the customers to draw.
        let mut start_index = self.scroll_pos.saturating_sub(self.rows * 2 / 3);
        let mut end_index = (start_index + self.rows).min(self.filtered.len());

        // if we've scrolled past the end of the buffer, adjust start_index to show the last page
        // of customers
        if end_index == self.filtered.len() && start_index + self.rows > end_index {
            start_index = end_index.saturating_sub(self.rows);
        }

        if self.scroll_pos < self.rows * 2 / 3 {
            start_index = 0;
            end_index = self.rows.min(self.filtered.len());
        }

        for i in start_index..end_index {
            let customer_index = self.filtered[i];  // get the index of the customer
            let customer = &self.buffer[customer_index];  // look up the customer in `buffer`
            if self.scroll_pos == i {
                stdout().queue(SetColors(Colors::new(self.color_scheme.dark_black, self.color_scheme.magenta)))?;
            } else {
                stdout().queue(SetColors(Colors::new(self.color_scheme.magenta, self.color_scheme.dark_black)))?;
            }
            stdout().queue(Clear(ClearType::CurrentLine))?;
            stdout().queue(Print(format!("{}: {}", customer_index, customer.name)))?;
            stdout().queue(MoveToNextLine(1))?;
        }
        stdout().queue(RestorePosition)?;
        stdout().flush()?;

        self.draw_scroll_bar()?;

        Ok(())
    }


    fn draw_scroll_bar(&self) -> io::Result<()> {
        stdout().queue(SavePosition)?;
        stdout().queue(MoveTo(10, 10))?;
        stdout().queue(Print(self.calculate_scrollbar_handle_position().to_string()))?;

        let scrollbar_pos = self.calculate_scrollbar_handle_position();

        for i in 1..=self.rows {
            stdout().queue(MoveTo(self.cols as u16-1,  i as u16))?;
            if i == scrollbar_pos {
                stdout().queue(SetColors(Colors::new(self.color_scheme.cyan, self.color_scheme.dark_black)))?;
                stdout().queue(Print(" "))?;
                stdout().queue(SetColors(Colors::new(self.color_scheme.white, self.color_scheme.black)))?;
            } else {
                stdout().queue(SetColors(Colors::new(self.color_scheme.dark_black, self.color_scheme.cyan)))?;
                stdout().queue(Print("░"))?;
                stdout().queue(SetColors(Colors::new(self.color_scheme.white, self.color_scheme.black)))?;
            }
        }
        stdout().queue(RestorePosition)?;
        stdout().flush()?;

        Ok(())

    }
    fn calculate_scrollbar_handle_position(&self) -> usize {
        let proportion = self.scroll_pos as f64 / (self.filtered.len() - 1) as f64;
        (proportion * (self.rows - 1) as f64).round() as usize + 1
    }
    pub fn get_results_count(&self) -> usize {
        self.filtered.len()
    }
    pub fn move_up(&mut self) -> io::Result<()> {
        if self.scroll_pos > 0 {
            self.scroll_pos -= 1;
        }
        self.draw()?;

        Ok(())
    }
    pub fn move_down(&mut self) -> io::Result<()> {
        if self.scroll_pos < self.filtered.len() - 1 {
            self.scroll_pos += 1;
        }
        self.draw()?;

        Ok(())
    }
}

