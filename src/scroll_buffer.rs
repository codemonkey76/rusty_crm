use crate::colors::ColorScheme;
use crate::customer::Customer;
use std::io::{self, Write, stdout};
use crossterm::cursor::{SavePosition, RestorePosition, MoveTo, MoveToNextLine};
use crossterm::style::{Print, SetColors, Colors };
use crossterm::terminal::{size, Clear, ClearType};
use crossterm::QueueableCommand;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::phone::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub phone_ip: String,
    pub password: String,
    pub line: PhoneLine
}

pub struct ScrollBuffer {
    buffer: Vec<Customer>,
    config: Config,
    filter: String,
    filtered: Vec<usize>,
    scroll_pos: usize,
    rows: usize,
    cols: usize,
    color_scheme: ColorScheme,
    phone: Option<Phone>
}


impl ScrollBuffer {
    pub fn new(color_scheme: ColorScheme) -> Result<Self, io::Error> {
        let size = size()?;
        let (cols, rows) = (size.0 as usize, size.1 as usize - 2);

        Ok(ScrollBuffer {
            buffer: Vec::new(),
            config: Config {
                phone_ip: "".to_string(),
                password: "".to_string(),
                line: PhoneLine::Line1
            },
            filtered: Vec::new(),
            filter: String::new(),
            scroll_pos: 0,
            cols,
            rows,
            color_scheme,
            phone: None
        })
    }

    pub fn delete_customer(&mut self) -> io::Result<()> {
        self.buffer.remove(self.scroll_pos);
        self.set_filter(self.filter.clone())?;

        Ok(())
    }

    pub fn splash_screen(&mut self) -> io::Result<()> {
        self.clear()?;
        stdout().queue(MoveTo(0, 1))?;
        self.set_colors()?;
        stdout().queue(Print("Welcome to Rusy CRM"))?;
        stdout().queue(MoveToNextLine(2))?;
        stdout().queue(Print("Shortcut Keys:"))?;
        stdout().queue(MoveToNextLine(1))?;
        stdout().queue(Print(" Ctrl+Q -> Quit Program"))?;
        stdout().queue(MoveToNextLine(1))?;
        stdout().queue(Print(" Ctrl+A -> Add Customer"))?;
        stdout().queue(MoveToNextLine(1))?;
        stdout().queue(Print(" Ctrl+E -> Edit Customer"))?;
        stdout().queue(MoveToNextLine(1))?;
        stdout().queue(Print(" Ctrl+D -> Delete Customer"))?;
        stdout().queue(MoveToNextLine(2))?;

        stdout().queue(Print("Press SPACE to continue"))?;
        stdout().flush()?;

        Ok(())
    }

    pub fn add_customer(&mut self, customer: Customer) {
        self.buffer.push(customer);
    }

    pub fn update_customer(&mut self, customer: Customer) {
        if self.get_selected_customer().is_some() {
            self.buffer[self.filtered[self.scroll_pos]] = customer;
        } else {
            self.buffer.push(customer);
        }
    }

    pub fn load_sample_data(&mut self) {
        self.buffer = Customer::generate(1000);
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
    }

    pub fn load_config(&mut self, config_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let contents = std::fs::read_to_string(config_path)?;

        self.config = toml::from_str(&contents)?;

        log::info!("Loaded config: {:?}", self.config);

        self.phone = Some(Phone::new(self.config.phone_ip.clone(), self.config.password.clone(), self.config.line));

        Ok(())
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

    pub async fn dial_customer(&self) -> io::Result<()> {
        log::info!("Dialling customer");
        if let Some(customer) = self.get_selected_customer() {
            log::info!("Dialling customer: {:?}", customer);
            if let Some(phone) = &customer.phone {
                log::info!("Dialling phone: {}", phone);
                if let Some(p) = &self.phone {
                    log::info!("Phone is initialized..., sending keys");
                    p.send_keys(self.get_phone_keys(&phone)).await;
                }
            }
        }

        Ok(())
    }

    pub fn get_phone_keys(&self, number: &str) -> Vec<PhoneKey> {
        let mut keys = Vec::new();

        for c in number.chars() {
            let key = match c {
                '0' => PhoneKey::KeypadKey(KeypadKey::Zero),
                '1' => PhoneKey::KeypadKey(KeypadKey::One),
                '2' => PhoneKey::KeypadKey(KeypadKey::Two),
                '3' => PhoneKey::KeypadKey(KeypadKey::Three),
                '4' => PhoneKey::KeypadKey(KeypadKey::Four),
                '5' => PhoneKey::KeypadKey(KeypadKey::Five),
                '6' => PhoneKey::KeypadKey(KeypadKey::Six),
                '7' => PhoneKey::KeypadKey(KeypadKey::Seven),
                '8' => PhoneKey::KeypadKey(KeypadKey::Eight),
                '9' => PhoneKey::KeypadKey(KeypadKey::Nine),
                '*' => PhoneKey::KeypadKey(KeypadKey::Star),
                '#' => PhoneKey::KeypadKey(KeypadKey::Hash),
                ' ' => continue, // Skip spaces
                _ => panic!("Invalid character in phone number: {}", c)
            };
            keys.push(key);
        }
        keys

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
        if ! self.filtered.is_empty() {
            let customer_index = self.filtered[self.scroll_pos];
            return Some(&self.buffer[customer_index]);
        }
        None
    }
}

