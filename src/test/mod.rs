mod fixture;

use crate::client::Sendable;
use crate::Client;
use crate::api::*;

const SENDGRID_API_KEY: &str = "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";

fn create_client() -> Client {
    let _ = env_logger::builder().is_test(true).try_init();

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

    let request = MailSendRequest {
        from: Address {
            email: "nick@infinyon.com".to_owned()
        },
        subject: "Crate Test Email".to_owned(),
        personalizations: vec![
            Personalization {
                to: vec![Address {
                    email: "nick+cratetest@infinyon.com".to_owned()
                }]
            }
        ],
        ..Default::default()
    };

    let _response = request.send(&client).await.unwrap();
}

#[async_std::test]
async fn test_list_templates() {
    let client = create_client();

    let request = ListTemplatesRequest::default();

    let response = request.send(&client).await.unwrap();

    assert_eq!("template1_id", response.result[0].id);
}
