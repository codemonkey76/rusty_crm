use serde::{Serialize, Deserialize};

pub struct Phone {
    address: String,
    password: String,
    client: Option<reqwest::blocking::Client>,
    _line: PhoneLine
}

const BASE_URL: &str = "cgi-bin/api-";

impl Phone {
    pub fn new(address: String, password: String, _line: PhoneLine) -> Phone {

        let client_result = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(true)
            .build();

        let client = match client_result {
            Ok(client) => {
                log::info!("{:?}", client);
                Some(client)
            },
            Err(e) => {
                log::error!("Failed to build the client: {:?}", e);
                None
            },
        };

        log::info!("Constructing Phone struct...");

        Phone {
            address,
            password,
            client,
            _line
        }
    }

    pub async fn get_line_status(&self) -> Option<String> {
        if let Some(client) = &self.client {
            let url = format!("https://{}/{}{}?passcode={}",
                              self.address,
                              BASE_URL,
                              "get_line_status",
                              self.password);

            let res = client.get(&url).send().ok()?.text().ok()?;

            return Some(res);
        }
        None
    }

    pub fn get_phone_status(&self) {}

    pub fn phone_operation(&self, _operation: PhoneOperation) {}

    pub fn system_operation(&self, _operation: SystemOperation) {}

    pub fn send_key(&self, _key: PhoneKey) {}

    fn keypad_key_to_string(&self, key: KeypadKey) -> String {
        match key {
            KeypadKey::Zero => "0".to_owned(),
            KeypadKey::One => "1".to_owned(),
            KeypadKey::Two => "2".to_owned(),
            KeypadKey::Three => "3".to_owned(),
            KeypadKey::Four => "4".to_owned(),
            KeypadKey::Five => "5".to_owned(),
            KeypadKey::Six => "6".to_owned(),
            KeypadKey::Seven => "7".to_owned(),
            KeypadKey::Eight => "8".to_owned(),
            KeypadKey::Nine => "9".to_owned(),
            _ => "".to_owned()
        }
    }

    pub fn send_keys(&self, keys: Vec<PhoneKey>) {
        // Make a post request, sending the keys
        if let Some(client) = &self.client {
            let url = format!("https://{}/{}{}",
                              self.address,
                              BASE_URL,
                              "send_key");
            
            for key in keys {
                let key = match key {
                    PhoneKey::KeypadKey(c) => self.keypad_key_to_string(c),
                    PhoneKey::Send => "SEND".to_owned(),
                    _ => continue
                };

                let params = [("password", self.password.clone()), ("keys", key.to_owned())];

                client.post(&url).form(&params).send().expect("Got an error posting to endpoint");
            }
        }
    }

}

pub enum PhoneOperation {
    EndCall,
    HoldCall,
    AcceptCall,
    RejectCall,
    Cancel
}

pub enum SystemOperation {
    Reboot,
    Reset
}

pub enum PhoneKey {
    Speaker,
    Transfer,
    VolUp,
    VolDown,
    Mute,
    Hold,
    KeypadKey(KeypadKey),
    Line(PhoneLine),
    Conference,
    VoiceMail,
    Headset,
    DoNotDisturb,
    Send,
    SoftKey(SoftKey),
    MultiPurposeKey(MultiPurposeKey),
    Star,
    OnHook,
    OffHook,
    OkButton,
    Lock,
    Unlock,
    Up,
    Down,
    Left,
    Right,
}

pub enum MultiPurposeKey {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key10,
    Key11,
    Key12,
    Key13,
    Key14,
    Key15,
    Key16,
    Key17,
    Key18,
    Key19,
    Key20,
    Key21,
    Key22,
    Key23,
    Key24
}

pub enum SoftKey {
    Key1,
    Key2,
    Key3,
    Key4,
    Left,
    Right
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum PhoneLine {
    Line1,
    Line2,
    Line3,
    Line4,
    Line5,
    Line6,
    Line7,
    Line8
}

pub enum KeypadKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Star,
    Hash
}
