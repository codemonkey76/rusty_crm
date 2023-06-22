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
        let filter_clone = self.filter.clone().to_lowercase();
        self.filtered = self.buffer.iter().enumerate().filter(|(_, c)| c.name.to_lowercase().contains(&filter_clone)).map(|(i, _)| i).collect();
        log::info!("Filtered count: {}", self.filtered.len());
        self.scroll_pos = 0;
        self.draw()?;

        Ok(())
    }

    pub fn clear(&self) -> io::Result<()> {
        stdout().queue(MoveTo(0, 1))?;
        for _ in 0..self.rows {
            stdout().queue(Clear(ClearType::CurrentLine))?;
            stdout().queue(MoveToNextLine(1))?;
        }
        stdout().flush()?;

        Ok(())
    }
    pub fn draw(&self) -> io::Result<()> {
        log::info!("Starting function ScrollBuffer::draw");
        self.clear()?;
        stdout().queue(SavePosition)?;
        stdout().queue(MoveTo(0, 1))?;

        // Calculate the start and end index for the customers to draw.
        log::info!("Calculating start and end index");
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

        log::info!("Starting loop to draw customers in filtered vector");

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
        log::info!("Finished drawing customers");

        if self.filtered.len() > self.rows {
            self.draw_scroll_bar()?;
        }

        Ok(())
    }


    fn draw_scroll_bar(&self) -> io::Result<()> {
        log::info!("Drawing scrollbar");
        stdout().queue(SavePosition)?;

        log::info!("Calculating scrollbar position");
        let scrollbar_pos = self.calculate_scrollbar_handle_position();

        log::info!("Looping through rows to draw scrollbar");
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
        log::info!("Flushing commands");
        stdout().flush()?;

        log::info!("Finished drawing scrollbar");
        Ok(())

    }
    fn calculate_scrollbar_handle_position(&self) -> usize {
        if self.filtered.len() <= self.rows {
            return 1;
        }
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

