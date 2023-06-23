//use crate::contact::Contact;
//use crate::note::Note;
use fake::faker::phone_number::en::PhoneNumber;
use fake::faker::name::en::Name;
use fake::Fake;
use std::path::PathBuf;
use std::io::Error;
use std::fs::{self,File};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    pub name: String,
    //pub contacts: Vec<Contact>,
    //pub notes: Vec<Note>,
    pub phone: Option<String>,
    pub voip_server: Option<String>
}

impl Customer {
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
            //contacts: Vec::new(),
            //notes: Vec::new(),
            phone: Some(PhoneNumber().fake::<String>()),
            voip_server: None
        }
    }
}
