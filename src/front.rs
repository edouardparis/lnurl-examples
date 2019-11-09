use hyper::{Body, Method, Request, Response, StatusCode};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::error;
use crate::faucet::Faucet;

static INDEX: &[u8] = b"<h1> LNURL examples </h1> <a href=\"/faucet\">faucet</a>";
static NOTFOUND: &[u8] = b"Not Found";
static FAUCET: &[u8] = b"faucet";

pub fn routes(
    req: Request<Body>,
    faucet: Arc<Faucet>,
) -> Result<Response<Body>, error::GenericError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => {
            let count = faucet.counter.fetch_add(1, Ordering::AcqRel);
            Ok(Response::new(Body::from(format!("Request #{}", count))))
        }
        (&Method::GET, "/faucet") => Ok(Response::new(FAUCET.into())),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}
