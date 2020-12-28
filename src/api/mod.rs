// mod client;
// mod message;
// mod personalization;
// mod requests;

// pub use client::Client;
// pub use message::{Address, Attachment, Content, Message};
// pub use personalization::Personalization;
// pub use requests::*;

mod mail_send;

pub use mail_send::*;