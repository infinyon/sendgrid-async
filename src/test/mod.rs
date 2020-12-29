mod fixture;

use crate::client::Sendable;
use crate::Client;
use crate::api::*;

const SENDGRID_API_KEY: &str = "TEST_API_KEY";

fn create_client() -> Client {
    let _ = env_logger::builder().is_test(true).try_init();

    let base_url = fixture::run().unwrap();

    Client::builder()
        .base_url(base_url)
        .key(SENDGRID_API_KEY)
        .build()
        .unwrap()
}

#[async_std::test]
async fn test_list_mail_send() {
    let client = create_client();

    let request = MailSendRequest::default();

    let _response = request.send(&client).await.unwrap();
}

#[async_std::test]
async fn test_list_templates() {
    let client = create_client();

    let request = ListTemplatesRequest::default();

    let response = request.send(&client).await.unwrap();

    assert_eq!("template1_id", response.result[0].id);
}
