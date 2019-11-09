use hyper::{Body, Method, Request, Response, StatusCode};
use std::sync::{atomic::AtomicUsize, Arc};

use crate::error;
use crate::faucet::Faucet;

static INDEX: &[u8] = b"<h1> Welcome to api !</h1>";
static NOTFOUND: &[u8] = b"Not Found";

pub fn routes(
    req: Request<Body>,
    faucet: Arc<Faucet>,
) -> Result<Response<Body>, error::GenericError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/lnurl/withdrawal") => Ok(Response::new(INDEX.into())),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}