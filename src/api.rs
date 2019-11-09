use hyper::{header, Body, Method, Request, Response, StatusCode};
use std::sync::Arc;

use crate::error;
use crate::lnurl;
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
    if faucet.is_empty() {
        let res = serde_json::to_string(
            &lnurl::Response::Error{reason: "faucet is empty".to_string()}
        ).unwrap();
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(res)).unwrap())
    };
    let withdrawal = serde_json::to_string(&lnurl::Withdrawal {
        default_description: "ln-faucet".to_string(),
        callback: faucet.callback.clone(),
        k1: "secret".to_string(),
        max_withdrawable: 1000,
        min_withdrawable: Some(10),
        tag: lnurl::Tag::WithdrawalRequest,
    })
    .unwrap();
Ok(Response::builder()
    .header(header::CONTENT_TYPE, "application/json")
    .body(Body::from(withdrawal)).unwrap())
}

pub async fn create_withdrawal(req: Request<Body>, faucet: Arc<Faucet>) -> Result<Response<Body>, error::GenericError> {
    let res = serde_json::to_string(
        &lnurl::Response::Ok
    ).unwrap();
    return Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(res)).unwrap())
}
