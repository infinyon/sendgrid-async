use data_encoding::BASE64;
use serde::Serialize;
/// The main structure for a V3 API mail send call. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Default, Serialize)]
pub struct Message {
    from: Address,
    subject: String,
    personalizations: Vec<super::Personalization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<Content>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    attachments: Option<Vec<Attachment>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    template_id: Option<String>,
}

impl Message {
    /// Construct a new V3 message.
    pub fn new() -> Message {
        Message::default()
    }

    /// Set the from address.
    pub fn set_from(mut self, from: Address) -> Message {
        self.from = from;
        self
    }

    /// Set the subject.
    pub fn set_subject(mut self, subject: &str) -> Message {
        self.subject = String::from(subject);
        self
    }

    /// Set the template id.
    pub fn set_template_id(mut self, template_id: &str) -> Message {
        self.template_id = Some(String::from(template_id));
        self
    }

    /// Add content to the message.
    pub fn add_content(mut self, c: Content) -> Message {
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
    pub fn add_personalization(mut self, p: super::Personalization) -> Message {
        self.personalizations.push(p);
        self
    }

    /// Add an attachment to the message.
    pub fn add_attachment(mut self, a: Attachment) -> Message {
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
/// An email with a required address and an optional name field.
#[derive(Clone, Default, Serialize)]
pub struct Address {
    email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl Address {
    /// Construct a new email type.
    pub fn new() -> Address {
        Address::default()
    }

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

/// The body of an email with the content type and the message.
#[derive(Clone, Default, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    value: String,
}

impl Content {
    /// Construct a new content type.
    pub fn new() -> Content {
        Content::default()
    }

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
#[derive(Default, Serialize)]
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
    /// Construct a new attachment for this message.
    pub fn new() -> Attachment {
        Attachment::default()
    }

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
