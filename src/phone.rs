use reqwest;
use reqwest::ClientBuilder;
use serde::{Serialize, Deserialize};

pub struct Phone {
    address: String,
    password: String,
    client: Option<reqwest::Client>,
    line: PhoneLine
}

const BASE_URL: &'static str = "cgi-bin/api-";

impl Phone {
    pub fn new(address: String, password: String, line: PhoneLine) -> Phone {
        let client = ClientBuilder::new()
            .danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true)
            .build()
            .ok();

        Phone {
            address,
            password,
            client,
            line
        }
    }

    pub async fn get_line_status(&self) -> Option<String> {
        if let Some(client) = &self.client {

            let url = format!("https://{}/{}{}?passcode={}",
                              self.address,
                              BASE_URL,
                              "get_line_status",
                              self.password);

            let res = client.get(&url).send().await.ok()?.text().await.ok()?;

            return Some(res);
        }
        None
    }

    pub fn get_phone_status(&self) {
    }

    pub fn phone_operation(&self, operation: PhoneOperation) {
    }

    pub fn system_operation(&self, operation: SystemOperation) {
    }

    pub fn send_key(&self, key: PhoneKey) {
    }

    pub async fn send_keys(&self, keys: Vec<PhoneKey>) {
        // Make a post request, sending the keys
        if let Some(client) = &self.client {
            let url = format!("https://{}/{}{}",
                              self.address,
                              BASE_URL,
                              "send_key");
            
            for key in keys {
                let key = match key {
                    PhoneKey::KeypadKey(c) => self.keypad_key_to_string(),
                    PhoneKey::KeypadKey(One) => "1",
                    PhoneKey::KeypadKey(Two) => "2",
                    PhoneKey::KeypadKey(Three) => "3",
                    PhoneKey::KeypadKey(Four) => "4",
                    PhoneKey::KeypadKey(Five) => "5",
                    PhoneKey::KeypadKey(Six) => "6",
                    PhoneKey::KeypadKey(Seven) => "7",
                    PhoneKey::KeypadKey(Eight) => "8",
                    PhoneKey::KeypadKey(Nine) => "8",
                    _ => continue
                };

                let params = [("password", self.password.clone()), ("keys", key.to_owned())];

                let res = client.post(&url).form(&params).send().await.ok().unwrap().text().await.ok();
            }
        }
    }

}

enum PhoneOperation {
    EndCall,
    HoldCall,
    AcceptCall,
    RejectCall,
    Cancel
}

enum SystemOperation {
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
