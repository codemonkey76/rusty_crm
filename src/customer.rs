use crate::contact::Contact;
use crate::note::Note;


struct Customer {
    name: String,
    contacts: Vec<Contact>,
    notes: Vec<Note>,
    phone: Option<String>,
    voip_server: Option<String>
}
