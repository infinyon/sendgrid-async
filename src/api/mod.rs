// mod client;
// mod message;
// mod personalization;
// mod requests;

// pub use client::Client;
// pub use message::{Address, Attachment, Content, Message};
// pub use personalization::Personalization;
// pub use requests::*;

mod mail_send;
mod transactional_template;
mod transactional_template_version;
mod paging;
mod error;

pub use mail_send::*;
pub use transactional_template::*;
pub use transactional_template_version::*;
pub use error::{ErrorReponse, Error};