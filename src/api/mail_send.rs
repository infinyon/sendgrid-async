use std::collections::HashMap;
use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::client::Sendable;

/// This endpoint allows you to send email over SendGrid’s v3 Web API, the most recent version of our API.
#[derive(Serialize)]
struct MailSendRequest {
    #[serde(flatten)]
    message: Message,
}

impl Sendable for MailSendRequest {
    type Response = ();
    type ErrorResponse = ();

    const METHOD: http_types::Method = http_types::Method::Post;

    fn rel_path(&self) -> Cow<'static, str> {
        Cow::Borrowed("mail/send")
    }

}

/// The main structure for the mail send request. This is composed of many other smaller
/// structures used to add lots of customization to your message.
#[derive(Default, Serialize)]
struct Message {
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

/// An email with a required address and an optional name field.
#[derive(Clone, Default, Serialize)]
pub struct Address {
    email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

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
    dynamic_template_data: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    send_at: Option<u64>,
}

/// The body of an email with the content type and the message.
#[derive(Clone, Default, Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    content_type: String,
    value: String,
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
