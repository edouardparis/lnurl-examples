use hyper::{header, Body, Method, Request, Response, StatusCode};
use std::sync::Arc;
use std::collections::HashMap;
use lnurl;

use crate::error;
use crate::faucet::Faucet;

static NOTFOUND: &[u8] = b"Not Found";

pub async fn routes(
    req: Request<Body>,
    faucet: Arc<Faucet>,
) -> Result<Response<Body>, error::GenericError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/lnurl/withdrawal") => lnurl_withdrawal(faucet),
        (&Method::GET, "/api/withdrawals/create") => create_withdrawal(req, faucet).await,
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}

pub fn lnurl_withdrawal(faucet: Arc<Faucet>) ->Result<Response<Body>, error::GenericError> {
    if faucet.is_locked() {
        return lnurl_error("faucet is locked");
    }
    let withdrawal = serde_json::to_string(&lnurl::Withdrawal {
        default_description: "ln-faucet".to_string(),
        callback: faucet.callback.clone(),
        k1: "secret".to_string(),
        max_withdrawable: faucet.amount_max_withdrawable,
        min_withdrawable: Some(faucet.amount_min_withdrawable),
        tag: lnurl::Tag::WithdrawRequest,
    })
    .unwrap();
    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(withdrawal)).unwrap())
}

pub async fn create_withdrawal(req: Request<Body>, faucet: Arc<Faucet>) -> Result<Response<Body>, error::GenericError> {
    info!("{:?}", req.uri().query());
    let params: HashMap<String, String> = match req.uri().query() {
        Some(q) =>  q.split("&").map(|kv| kv.split('='))
            .map(|mut kv| (kv.next().unwrap().into(), kv.next().unwrap().into())).collect(),
        None => return lnurl_error("pr is required"),
    };
    let invoice = match params.get("pr") {
        None => return lnurl_error("pr is required"),
        Some(i) => i,
    };

    if let Err(e) = faucet.pay_invoice(invoice.to_string()).await {
        match e {
            error::Error::BadInvoice => return lnurl_error("bad pr"),
            _ => {error!("{:?}", e); return lnurl_error("internal_error")},
        }
    }
    faucet.lock();

    lnurl_ok()
}

fn lnurl_error(err: &str) -> Result<Response<Body>, error::GenericError> {
    let res = serde_json::to_string(
        &lnurl::Response::Error{reason: err.to_string()}
    ).unwrap();
    return Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(res)).unwrap())
}

fn lnurl_ok() -> Result<Response<Body>, error::GenericError> {
    let res = serde_json::to_string(
        &lnurl::Response::Ok
    ).unwrap();
    return Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(res)).unwrap())
}
