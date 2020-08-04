use serde::Serialize;
use std::collections::HashMap;

/// A personalization block for a V3 message. It has to at least contain one super::Address as a to
/// address. All other fields are optional.
#[derive(Default, Serialize)]
pub struct Personalization {
    to: Vec<super::Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<super::Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<Vec<super::Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_args: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_template_data: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<u64>,
}

impl Personalization {
    /// Construct a new personalization block for this message.
    pub fn new() -> Personalization {
        Personalization::default()
    }

    /// Add a to field.
    pub fn add_to(mut self, to: super::Address) -> Personalization {
        self.to.push(to);
        self
    }

    /// Add a CC field.
    pub fn add_cc(mut self, cc: super::Address) -> Personalization {
        match self.cc {
            None => {
                let mut ccs = Vec::new();
                ccs.push(cc);
                self.cc = Some(ccs);
            }
            Some(ref mut c) => {
                c.push(cc);
            }
        }
        self
    }

    /// Add a BCC field.
    pub fn add_bcc(mut self, bcc: super::Address) -> Personalization {
        match self.bcc {
            None => {
                let mut bccs = Vec::new();
                bccs.push(bcc);
                self.bcc = Some(bccs);
            }
            Some(ref mut b) => {
                b.push(bcc);
            }
        }
        self
    }

    /// Add a headers field.
    pub fn add_headers(mut self, headers: HashMap<String, String>) -> Personalization {
        match self.headers {
            None => {
                let mut h = HashMap::new();
                for (name, value) in headers {
                    h.insert(name, value);
                }
                self.headers = Some(h);
            }
            Some(ref mut h) => {
                h.extend(headers);
            }
        }
        self
    }

    /// Add a dynamic template data field.
    pub fn set_dynamic_template_data(
        mut self,
        dynamic_template_data: impl Serialize,
    ) -> Result<Personalization, serde_json::Error> {
        let value = serde_json::to_value(dynamic_template_data)?;
        self.dynamic_template_data = Some(value);
        Ok(self)
    }
}
