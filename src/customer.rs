use fake::faker::phone_number::en::PhoneNumber;
use fake::faker::name::en::Name;
use fake::Fake;
use std::path::PathBuf;
use std::io::Error;
use std::fs::{self,File};
use serde::{Serialize, Deserialize};
use serde_json;
use std::fmt::{self, Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub name: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub voip_server: Option<String>
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            name: String::new(),
            contact_name: None,
            phone: None,
            voip_server: None
        }
    }
    pub fn load_customers(file_path: PathBuf) -> Result<Vec<Customer>, Error> {
        let file = File::open(file_path)?;

        let customers: Vec<Customer> = serde_json::from_reader(file)?;

        Ok(customers)
    }
    pub fn save_customers(customers: &[Customer], file_path: PathBuf) -> Result<(), Error> {
        log::info!("Saving customers to {}", file_path.display());
        // Ensure the directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // Open the file
        let file = File::create(&file_path).unwrap();

        // Serialize the customers into the file
        serde_json::to_writer_pretty(file, customers).unwrap();

        Ok(())
    }    
    pub fn generate(n: usize) -> Vec<Customer> {
        (1..n).map(|_| Customer::sample()).collect()
    }
    pub fn sample() -> Customer {
        Customer {
            name: Name().fake::<String>(),
            contact_name: Some(Name().fake::<String>()),
            phone: Some(PhoneNumber().fake::<String>()),
            voip_server: None
        }
    }
    pub fn set_company_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_contact_name(&mut self, contact_name: String) {
        self.contact_name = Some(contact_name);
    }
    pub fn set_phone_number(&mut self, phone: String) {
        self.phone = Some(phone);
    }
    pub fn get_company_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_contact_name(&self) -> String {
        self.contact_name.clone().unwrap_or(String::new())
    }
    pub fn get_phone_number(&self) -> String {
        self.phone.clone().unwrap_or(String::new())
    }
}
impl Display for Customer {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} - {}", 
               if self.name.is_empty() { "(none)" } else { &self.name },
               self.contact_name.as_ref().map(|s| s.as_str()).unwrap_or("(none)"),
               self.phone.as_ref().map(|s| s.as_str()).unwrap_or("(none)"))
    }
}
