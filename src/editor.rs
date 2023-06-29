use crate::colors::ColorScheme;
use crate::line_buffer::LineBuffer;
use crate::scroll_buffer::ScrollBuffer;
use crate::status_line::StatusLine;
use crate::utils::RawMode;
use crate::customer::Customer;
use crossterm::event::{read, poll, Event, KeyCode, KeyModifiers};
use std::io;
use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub enum EditorMode {
    Normal,
    SplashScreen,
    AddCompanyName,
    AddContactName,
    AddPhoneNumber,
    EditCompanyName,
    EditContactName,
    EditPhoneNumber,
    Delete
}
pub struct Editor {
    pub file_path: PathBuf,
    pub config_path: PathBuf,
    pub line_buffer: LineBuffer,     // The line buffer
    pub scroll_buffer: ScrollBuffer, // The scroll buffer
    pub status_line: StatusLine,     // The status line
    pub mode: EditorMode,            // The editor mode
    pub color_scheme: ColorScheme,   // The color scheme
    temp_customer: Customer,         // The temporary customer
    no_splash: bool,
    sample_data: bool,
    _raw_mode: RawMode,              // The raw mode
}

impl Editor {
    pub fn new(file_path: PathBuf, config_path: PathBuf, no_splash: bool, sample_data: bool) -> Result<Editor, std::io::Error> {
        let color_scheme = ColorScheme::new();
        let line_buffer = LineBuffer::new("Query: ".to_string(), color_scheme.clone());
        let scroll_buffer = ScrollBuffer::new(color_scheme.clone())?;
        let status_line = StatusLine::new(color_scheme.clone())?;

        let _raw_mode = RawMode::new()?;

        Ok(Editor {
            file_path,
            config_path,
            line_buffer,
            scroll_buffer,
            status_line,
            mode: EditorMode::SplashScreen,
            color_scheme,
            temp_customer: Customer::new(),
            no_splash,
            sample_data,
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
                            if ! self.sample_data {
                                self.save()?;
                            }
                            break;
                        },
                        KeyCode::Char('s') if event.modifiers.contains(KeyModifiers::CONTROL) => { if !self.sample_data { self.save()?; } },
                        KeyCode::Char('c') if event.modifiers.contains(KeyModifiers::CONTROL) => { self.call_customer()?; },
                        KeyCode::Char('a') if event.modifiers.contains(KeyModifiers::CONTROL) => { self.add_customer()?; },
                        KeyCode::Char('e') if event.modifiers.contains(KeyModifiers::CONTROL) => { self.edit_customer()?; },
                        KeyCode::Char('d') if event.modifiers.contains(KeyModifiers::CONTROL) => { self.delete_customer()?; },
                        KeyCode::Char(' ') => { 
                            if self.mode == EditorMode::SplashScreen {
                                self.set_mode(EditorMode::Normal)?;
                            }
                            self.add_key(' ')?;
                        }
                        KeyCode::Esc => { self.set_mode(EditorMode::Normal)?; },
                        KeyCode::Enter => { self.enter()?; },
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

    pub fn set_mode(&mut self, mode: EditorMode) -> io::Result<()> {
        log::info!("Setting mode to {:?}", mode);
        self.mode = mode;
        match self.mode {
            EditorMode::SplashScreen => {
                self.scroll_buffer.splash_screen()?;
                self.line_buffer.set_prompt("".to_string())?;
                self.status_line.set_message("".to_string())?;
                self.line_buffer.clear()?;
            },
            EditorMode::Normal => {
                self.line_buffer.set_prompt("Query: ".to_string())?;
                self.status_line.set_message("Normal Mode".to_string())?;
                self.line_buffer.clear()?;
                self.filter()?;
            },
            EditorMode::AddCompanyName => {
                self.line_buffer.set_prompt("Company name: ".to_string())?;
                self.status_line.set_message("Add Company".to_string())?;
                self.line_buffer.clear()?;
            },
            EditorMode::AddContactName => {
                self.line_buffer.set_prompt("Contact name: ".to_string())?;
                self.status_line.set_message("Add Contact".to_string())?;
                self.line_buffer.clear()?;
            },
            EditorMode::AddPhoneNumber => {
                self.line_buffer.set_prompt("Phone number: ".to_string())?;
                self.status_line.set_message("Add Phone Number".to_string())?;
                self.line_buffer.clear()?;
            },
            EditorMode::EditCompanyName => {
                log::info!("{:?}", self.scroll_buffer.get_selected_customer());
                if let Some(customer) = self.scroll_buffer.get_selected_customer() {
                    log::info!("Setting buffer to {}", customer.get_company_name());
                    self.line_buffer.set_buffer(customer.get_company_name())?;
                }
                self.line_buffer.set_prompt("Company name: ".to_string())?;
                self.status_line.set_message("Edit Company".to_string())?;
            },
            EditorMode::EditContactName => {
                if let Some(customer) = self.scroll_buffer.get_selected_customer() {
                    self.line_buffer.set_buffer(customer.get_contact_name())?;
                }
                self.line_buffer.set_prompt("Contact name: ".to_string())?;
                self.status_line.set_message("Edit Contact".to_string())?;
            },
            EditorMode::EditPhoneNumber => {
                if let Some(customer) = self.scroll_buffer.get_selected_customer() {
                    self.line_buffer.set_buffer(customer.get_phone_number())?;
                }
                self.line_buffer.set_prompt("Phone number: ".to_string())?;
                self.status_line.set_message("Edit Phone Number".to_string())?;
            },
            EditorMode::Delete => {
                self.line_buffer.set_prompt("Delete (y/n): ".to_string())?;
                self.status_line.set_message("DeleteMode".to_string())?;
            }
        }

        Ok(())
    }

    pub fn enter(&mut self) -> io::Result<()> {
        log::info!("Enter pressed");
        match self.mode {
            EditorMode::Normal | EditorMode::Delete => {
                self.set_mode(EditorMode::Normal)?;
                self.filter()?;
            },
            EditorMode::AddCompanyName => {
                self.temp_customer.set_company_name(self.line_buffer.get_string());
                self.set_mode(EditorMode::AddContactName)?;
            },
            EditorMode::AddContactName => {
                self.temp_customer.set_contact_name(self.line_buffer.get_string());
                self.set_mode(EditorMode::AddPhoneNumber)?;
            },
            EditorMode::AddPhoneNumber => {
                self.temp_customer.set_phone_number(self.line_buffer.get_string());
                self.scroll_buffer.add_customer(self.temp_customer.clone());
                self.set_mode(EditorMode::Normal)?;
                self.filter()?;
            },
            EditorMode::EditCompanyName => {
                self.temp_customer.set_company_name(self.line_buffer.get_string());
                self.set_mode(EditorMode::EditContactName)?;
            },
            EditorMode::EditContactName => {
                self.temp_customer.set_contact_name(self.line_buffer.get_string());
                self.set_mode(EditorMode::EditPhoneNumber)?;
            },
            EditorMode::EditPhoneNumber => {
                self.temp_customer.set_phone_number(self.line_buffer.get_string());
                self.scroll_buffer.update_customer(self.temp_customer.clone());
                self.set_mode(EditorMode::Normal)?;
                self.filter()?;
            },
            _ => {
                // Ignore the enter key
            }
        }

        Ok(())
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.scroll_buffer.load_config(self.config_path.clone()).unwrap();

        if self.sample_data {
            self.scroll_buffer.load_sample_data();
        } else {
            self.scroll_buffer.load_customers(self.file_path.clone());
        }

        self.filter()?;
        self.line_buffer.draw()?;
        self.scroll_buffer.draw()?;
        self.line_buffer.sync_caret()?;
        if self.no_splash {
            self.set_mode(EditorMode::Normal)?;
        } else {
            self.scroll_buffer.splash_screen()?;
        }

        self.scroll_buffer.dial_customer()?;
    
        Ok(())
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.scroll_buffer.save_customers(self.file_path.clone())?;
        self.status_line.set_message("Saved".to_string())?;

        Ok(())
    }

    pub fn filter(&mut self) -> io::Result<()> {
        self.scroll_buffer.set_filter(self.line_buffer.get_string())?;
        self.status_line.set_results_count(self.scroll_buffer.get_results_count())?;

        Ok(())
    }

    pub fn call_customer(&self) -> io::Result<()> {
        log::info!("Calling customer");
        self.scroll_buffer.dial_customer()?;

        Ok(())
    }

    pub fn add_key(&mut self, c: char) -> io::Result<()> {
        log::info!("Key pressed: {}", c);
        if self.mode == EditorMode::SplashScreen {
            
            // Ignore any key presses that aren't space bar
            return Ok(())
        }
        // Logic for handling delete mode
        if self.mode == EditorMode::Delete {
            log::info!("Delete mode");
            if c == 'y' {
                log::info!("Deleting customer");
                if self.scroll_buffer.get_selected_customer().is_some() {
                    log::info!("Found we have a valid selected customer");
                    self.scroll_buffer.delete_customer()?;
                    self.set_mode(EditorMode::Normal)?;
                    self.filter()?;
                }
            } else if c == 'n' {
                log::info!("Not deleting customer");
                self.set_mode(EditorMode::Normal)?;
                self.filter()?;
            }
            return Ok(());
        }
        log::info!("Not delete mode");
        log::info!("Mode: {:?}", self.mode);

        self.line_buffer.add(&c.to_string())?;
        match self.mode {
            EditorMode::Normal | EditorMode::Delete => {
                self.filter()?;
            },
            _ => {},
        }

        Ok(())
    }

    pub fn add_customer(&mut self) -> io::Result<()> {
        self.set_mode(EditorMode::AddCompanyName)?;
        Ok(())
    }

    pub fn edit_customer(&mut self) -> io::Result<()> {
        self.set_mode(EditorMode::EditCompanyName)?;
        Ok(())
    }

    pub fn delete_customer(&mut self) -> io::Result<()> {
        self.set_mode(EditorMode::Delete)?;
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
        if self.mode == EditorMode::Normal {
            self.filter()?;
        }
        self.line_buffer.sync_caret()?;


        Ok(())
    }

    pub fn backspace(&mut self) -> io::Result<()> {
        self.line_buffer.backspace()?;
        if self.mode == EditorMode::Normal {
            self.filter()?;
        }
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
