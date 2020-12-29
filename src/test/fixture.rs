use async_net::{IpAddr, SocketAddr};
use http_types::Body;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use serde_json::json;
use tide::{Request, Response};
use lazy_static::lazy_static;
use std::sync::{Arc, Once};

lazy_static! {
    static ref SOCK_ADDR: Arc<SocketAddr> = {
        let distribution = Uniform::new(2u16.pow(14), 2u16.pow(15));
        let port = thread_rng().sample(distribution);
        Arc::new(SocketAddr::new("127.0.0.1".parse::<IpAddr>().unwrap(), port))
    };
}

static INIT: Once = Once::new();

pub async fn run() -> tide::Result<String> {

    let sock_addr = *SOCK_ADDR.clone();

    INIT.call_once(|| {

        let mut app = tide::new();
        app.at("/mail/send").post(mail_send);

        async_std::task::spawn_local(app.listen(sock_addr));

    });

    Ok(format!("http://{}/v3/", sock_addr))
}

async fn mail_send(mut req: Request<()>) -> tide::Result {
    let body = Body::from_json(&json!({
        "success": true
    }))?;

    Ok(Response::builder(200).body(body).build())
}
