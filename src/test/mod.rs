mod fixture;

use crate::api::*;
use crate::client::Sendable;
use crate::Client;

const SENDGRID_API_KEY: &str =
    "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";

fn create_client() -> Client {
    let _ = tracing_subscriber::fmt::try_init();

    let base_url = fixture::run().unwrap();

    Client::builder()
        // .base_url(base_url)
        .key(SENDGRID_API_KEY)
        .build()
        .unwrap()
}

#[async_std::test]
async fn test_list_mail_send() {
    let client = create_client();

    let request = MailSendRequest::default()
        .set_from(Address::default().set_email("nick@infinyon.com"))
        .set_subject("Crate Test Email")
        .add_personalization(
            Personalization::default().add_to(Address::default().set_email("nick@infinyon.com")),
        )
        .add_content(
            Content::default()
                .set_content_type("text/plain")
                .set_value("Hello test"),
        );

    let _response = request.send(&client).await.unwrap();
}

#[async_std::test]
async fn test_list_templates() {
    let client = create_client();

    let request = ListTemplatesRequest::default();

    let response = request.send(&client).await.unwrap();

    assert_eq!("template1_id", response.result[0].id);
}
