//use crate::contact::Contact;
//use crate::note::Note;
use fake::faker::phone_number::en::PhoneNumber;
use fake::faker::name::en::Name;
use fake::Fake;

pub struct Customer {
    pub name: String,
    //pub contacts: Vec<Contact>,
    //pub notes: Vec<Note>,
    pub phone: Option<String>,
    pub voip_server: Option<String>
}

impl Customer {
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
