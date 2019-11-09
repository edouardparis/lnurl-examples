use hyper::{Body, Method, Request, Response, StatusCode};
use image::Luma;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::error;
use crate::faucet::Faucet;

static INDEX: &[u8] = b"<h1> LNURL examples </h1> <a href=\"/faucet\">faucet</a>";
static NOTFOUND: &[u8] = b"Not Found";

pub fn routes(
    req: Request<Body>,
    faucet: Arc<Faucet>,
) -> Result<Response<Body>, error::GenericError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/faucet") => {
            let count = faucet.counter.load(Ordering::Relaxed);
            let limit = faucet.limit.load(Ordering::Relaxed);
            Ok(Response::new(Body::from(format!(
                "Faucet: Limit {} Count {}",
                limit, count
            ))))
        }
        (&Method::GET, "/plus") => {
            let count = faucet.counter.fetch_add(1, Ordering::AcqRel);
            Ok(Response::new(Body::from(format!("Request #{}", count + 1))))
        }
        (&Method::GET, "/faucet/qrcode.png") => {
            Ok(Response::new(Body::from(faucet.qrcode.to_vec())))
        }
        (&Method::GET, "/") | (&Method::GET, "/index.html") => Ok(Response::new(INDEX.into())),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}