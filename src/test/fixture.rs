use async_net::{IpAddr, SocketAddr};
use http_types::Body;
use lazy_static::lazy_static;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use serde_json::json;
use std::sync::{Arc, Once};
use tide::{Request, Response};
use tracing::*;

lazy_static! {
    static ref SOCK_ADDR: Arc<SocketAddr> = {
        let distribution = Uniform::new(2u16.pow(14), 2u16.pow(15));
        let port = thread_rng().sample(distribution);
        Arc::new(SocketAddr::new(
            "127.0.0.1".parse::<IpAddr>().unwrap(),
            port,
        ))
    };
}

static INIT: Once = Once::new();

pub fn run() -> tide::Result<String> {
    let sock_addr = *SOCK_ADDR.clone();

    INIT.call_once(|| {
        let mut app = tide::new();
        app.at("/v3/templates").get(list_templates);
        app.at("/v3/mail/send").post(mail_send);

        async_std::task::spawn(app.listen(sock_addr));
        info!("test http server listening on {}", sock_addr);

        std::thread::sleep(std::time::Duration::from_secs(5));
    });

    Ok(format!("http://{}/v3/", sock_addr))
}

async fn mail_send(_req: Request<()>) -> tide::Result {
    Ok(Response::builder(202)
        .build())
}

async fn list_templates(_req: Request<()>) -> tide::Result {
    let body = Body::from_json(&json!({
        "result": [{
            "id": "template1_id",
            "name": "template1_name",
            "generation": "dynamic"
        }],
        "_metadata": {
            "self": "thispage"
        }
    }))?;

    Ok(Response::builder(200)
        .content_type("application/json")
        .body(body)
        .build())
}
