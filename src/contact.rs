pub struct Contact {
    name: String,
    phone: Option<String>,
    email: Option<String>,
}

const ERR_MSG: &str = "At least one of phone or email must be provided";

impl Contact {
    pub fn _new(name: String, phone: Option<String>, email: Option<String>) -> Result<Contact, &'static str> {
        if phone.is_none() && email.is_none() {
            return Err(ERR_MSG);
        }

        Ok(Contact { name, phone, email })
    }

    pub fn _set_phone(&mut self, phone: Option<String>) -> Result<(), &'static str>{
        if phone.is_none() && self.email.is_none() {
            return Err(ERR_MSG);
        }

        self.phone = phone;
        Ok(())
        }

    pub fn _set_email(&mut self, email: Option<String>) -> Result<(), &'static str> {
        if email.is_none() && self.phone.is_none() {
            return Err(ERR_MSG);
        }

        self.email = email;
        Ok(())
    }
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Phone: {}, Email: {}",
               self.name,
               self.phone.as_ref().unwrap_or(&"N/A".to_string()),
               self.email.as_ref().unwrap_or(&"N/A".to_string()))
    }
}

