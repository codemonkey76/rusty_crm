use crate::colors::ColorScheme;
use crate::customer::Customer;
use std::io::{self, Write, stdout};
use crossterm::cursor::{SavePosition, RestorePosition, MoveTo, MoveToNextLine};
use crossterm::style::{Print, SetColors, Colors };
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::QueueableCommand;
use std::path::PathBuf;

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

    pub fn add_customer(&mut self, customer: Customer) {
        self.buffer.push(customer);
    }
    pub fn update_customer(&mut self, customer: Customer) {
        if let Some(customer) = self.get_selected_customer() {
            self.buffer[self.filtered[self.scroll_pos]] = customer.clone();
        } else {
            self.buffer.push(customer);
        }
    }

    pub fn load_customers(&mut self, file_path: PathBuf) {
        match Customer::load_customers(file_path) {
            Ok(customers) => {
                self.buffer = customers;
            },
            Err(e) => {
                log::error!("Error loading customers: {}", e);
            }
        }
        //self.buffer = Customer::generate(1000);
    }
    pub fn save_customers(&mut self, file_path: PathBuf) -> io::Result<()> {
        Customer::save_customers(&self.buffer, file_path)
    }

    pub fn set_filter(&mut self, filter: String) -> io::Result<()> {
        self.filter = filter;
        let filter_clone = self.filter.clone().to_lowercase();
        self.filtered = self.buffer.iter().enumerate().filter(|(_, c)| {
            c.name.to_lowercase().contains(&filter_clone) ||
            c.contact_name.as_ref().unwrap_or(&"".to_string()).to_lowercase().contains(&filter_clone) ||
            c.phone.as_ref().unwrap_or(&"".to_string()).to_lowercase().contains(&filter_clone)
        }).map(|(i, _)| i).collect();
        
        log::info!("Filtered count: {}", self.filtered.len());
        self.scroll_pos = 0;
        self.draw()?;

        Ok(())
    }

    pub fn clear(&self) -> io::Result<()> {
        stdout().queue(MoveTo(0, 1))?;
        self.set_colors()?;
        for _ in 0..self.rows {
            stdout().queue(Clear(ClearType::CurrentLine))?;
            stdout().queue(MoveToNextLine(1))?;
        }
        stdout().flush()?;

        Ok(())
    }
    fn set_colors(&self) -> io::Result<()> {
        stdout().queue(SetColors(Colors::new(self.color_scheme.magenta, self.color_scheme.dark_black)))?;

        Ok(())
    }
    pub fn draw(&self) -> io::Result<()> {
        stdout().queue(SavePosition)?;
        self.clear()?;
        stdout().queue(MoveTo(0, 1))?;

        let mut start_index = self.scroll_pos.saturating_sub(self.rows * 2 / 3);
        let mut end_index = (start_index + self.rows).min(self.filtered.len());

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
            stdout().queue(Print(format!("{}", customer)))?;
            stdout().queue(MoveToNextLine(1))?;
        }
        stdout().queue(RestorePosition)?;
        stdout().flush()?;

        if self.filtered.len() > self.rows {
            self.draw_scroll_bar()?;
        }

        Ok(())
    }


    fn draw_scroll_bar(&self) -> io::Result<()> {
        stdout().queue(SavePosition)?;

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
    pub fn get_selected_customer(&self) -> Option<&Customer> {
        if self.filtered.len() > 0 {
            let customer_index = self.filtered[self.scroll_pos];
            return Some(&self.buffer[customer_index]);
        }
        None
    }
}

