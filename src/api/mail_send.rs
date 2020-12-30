use std::borrow::Cow;
use std::collections::HashMap;

use data_encoding::BASE64;
use http_types::{Body, Error as HttpError, Request};
use serde::Serialize;

use crate::client::Sendable;

/// This endpoint allows you to send email over SendGrid’s v3 Web API, the most recent version of our API.
/// The main structure for the mail send request. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Debug, Clone, Default, Serialize)]
pub struct MailSendRequest {
    from: Address,
    subject: String,
    personalizations: Vec<Personalization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<Content>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<String>,
}

impl MailSendRequest {
    /// Set the from address.
    pub fn set_from(mut self, from: Address) -> MailSendRequest {
        self.from = from;
        self
    }

    /// Set the subject.
    pub fn set_subject(mut self, subject: &str) -> MailSendRequest {
        self.subject = String::from(subject);
        self
    }

    /// Set the template id.
    pub fn set_template_id(mut self, template_id: &str) -> MailSendRequest {
        self.template_id = Some(String::from(template_id));
        self
    }

    /// Add content to the message.
    pub fn add_content(mut self, c: Content) -> MailSendRequest {
        match self.content {
            None => {
                let mut content = Vec::new();
                content.push(c);
                self.content = Some(content);
            }
            Some(ref mut content) => content.push(c),
        };
        self
    }

    /// Add a personalization to the message.
    pub fn add_personalization(mut self, p: Personalization) -> MailSendRequest {
        self.personalizations.push(p);
        self
    }

    /// Add an attachment to the message.
    pub fn add_attachment(mut self, a: Attachment) -> MailSendRequest {
        match self.attachments {
            None => {
                let mut attachments = Vec::new();
                attachments.push(a);
                self.attachments = Some(attachments);
            }
            Some(ref mut attachments) => attachments.push(a),
        };
        self
    }
}

impl Sendable for MailSendRequest {
    type Response = ();
    type ErrorResponse = super::ErrorReponse;

    const METHOD: http_types::Method = http_types::Method::Post;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Borrowed("mail/send")
    }

    fn modify_request(&self, req: &mut Request) -> Result<(), HttpError> {
        req.set_body(Body::from_json(self)?);
        Ok(())
    }
}

/// An email with a required address and an optional name field.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Address {
    email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Address {
    /// Set the address for this email.
    pub fn set_email(mut self, email: &str) -> Address {
        self.email = String::from(email);
        self
    }

    /// Set an optional name.
    pub fn set_name(mut self, name: &str) -> Address {
        self.name = Some(String::from(name));
        self
    }
}

/// A personalization block for a V3 message. It has to at least contain one Address as a to
/// address. All other fields are optional.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Personalization {
    to: Vec<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<Vec<Address>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    subject: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    substitutions: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dynamic_template_data: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    custom_args: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<u64>,
}

impl Personalization {
    /// Add a to field.
    pub fn add_to(mut self, to: Address) -> Personalization {
        self.to.push(to);
        self
    }

    /// Add a CC field.
    pub fn add_cc(mut self, cc: Address) -> Personalization {
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
    pub fn add_bcc(mut self, bcc: Address) -> Personalization {
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

    /// Add headers.
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

    /// Add substitutions.
    pub fn add_substitutions(mut self, substitutions: HashMap<String, String>) -> Personalization {
        match self.substitutions {
            None => {
                let mut h = HashMap::new();
                for (name, value) in substitutions {
                    h.insert(name, value);
                }
                self.substitutions = Some(h);
            }
            Some(ref mut h) => {
                h.extend(substitutions);
            }
        }
        self
    }

    /// Add dynamic template data.
    pub fn add_dynamic_template_data(mut self, dynamic_template_data: HashMap<String, String>) -> Personalization {
        match self.dynamic_template_data {
            None => {
                let mut h = HashMap::new();
                for (name, value) in dynamic_template_data {
                    h.insert(name, value);
                }
                self.dynamic_template_data = Some(h);
            }
            Some(ref mut h) => {
                h.extend(dynamic_template_data);
            }
        }
        self
    }

    /// Add custom args.
    pub fn add_custom_args(mut self, custom_args: HashMap<String, String>) -> Personalization {
        match self.custom_args {
            None => {
                let mut h = HashMap::new();
                for (name, value) in custom_args {
                    h.insert(name, value);
                }
                self.custom_args = Some(h);
            }
            Some(ref mut h) => {
                h.extend(custom_args);
            }
        }
        self
    }
}

/// The body of an email with the content type and the message.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    value: String,
}

impl Content {
    /// Set the type of this content.
    pub fn set_content_type(mut self, content_type: &str) -> Content {
        self.content_type = String::from(content_type);
        self
    }

    /// Set the corresponding message for this content.
    pub fn set_value(mut self, value: &str) -> Content {
        self.value = String::from(value);
        self
    }
}

/// An attachment block for a V3 message. Content and filename are required. If the
/// mime_type is unspecified, the email will use Sendgrid's default for attachments
/// which is 'application/octet-stream'.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Attachment {
    content: String,

    filename: String,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    mime_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    disposition: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content_id: Option<String>,
}

impl Attachment {
    /// The raw body of the attachment.
    pub fn set_content(mut self, c: &[u8]) -> Attachment {
        self.content = BASE64.encode(c);
        self
    }

    /// The base64 body of the attachment.
    pub fn set_base64_content(mut self, c: &str) -> Attachment {
        self.content = String::from(c);
        self
    }

    /// Sets the filename for the attachment.
    pub fn set_filename(mut self, filename: &str) -> Attachment {
        self.filename = filename.into();
        self
    }

    /// Set an optional mime type. Sendgrid will default to 'application/octet-stream'
    /// if unspecified.
    pub fn set_mime_type(mut self, mime: &str) -> Attachment {
        self.mime_type = Some(String::from(mime));
        self
    }
}
