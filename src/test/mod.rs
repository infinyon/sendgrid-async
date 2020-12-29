mod fixture;

use crate::client::Sendable;
use crate::Client;
use crate::ListTemplatesRequest;

const SENDGRID_API_KEY: &str = "TEST_API_KEY";

#[async_std::test]
async fn test_add() {
    let base_url = fixture::run().await.unwrap();
    // panic!(base_url);
    assert_eq!(3, 3);
}

#[async_std::test]
async fn test_add2() {
    let base_url = fixture::run().await.unwrap();
    // panic!(base_url);

    let client = Client::builder()
        .base_url(base_url)
        .key(SENDGRID_API_KEY)
        .build()
        .unwrap();

    let request = ListTemplatesRequest::default();

    let response = request.send(&client).await.unwrap();

    assert_eq!(3, 3);
}
