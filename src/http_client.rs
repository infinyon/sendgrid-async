use async_h1::client;
use http_types::{Error, Request, Response, StatusCode};
use log::*;

pub async fn execute(req: Request) -> Result<Response, Error> {
    debug!("executing request: {:#?}", req);

    let host = req
        .url()
        .host_str()
        .ok_or_else(|| Error::from_str(StatusCode::BadRequest, "missing hostname"))?
        .to_string();

    let scheme = req.url().scheme();
    if scheme != "https" {
        panic!(
            "{} scheme not is supported, only https is supported",
            scheme
        );
    }

    let addr = req
        .url()
        .socket_addrs(|| Some(443))?
        .into_iter()
        .next()
        .ok_or_else(|| Error::from_str(StatusCode::BadRequest, "missing valid address"))?;

    let raw_stream = async_std::net::TcpStream::connect(addr).await?;

    let tls_connector = async_tls::TlsConnector::default();
    let stream = tls_connector.connect(host, raw_stream).await?;

    let result = client::connect(stream, req).await;

    debug!("http result: {:#?}", result);

    result
}
