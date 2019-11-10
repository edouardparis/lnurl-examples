use hyper::{Body, Method, Request, Response, StatusCode};
use std::sync::{atomic::Ordering, Arc};

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
            if faucet.is_locked() {
                let time = faucet.remained_time.load(Ordering::Relaxed);
                return Ok(Response::new(Body::from(format!(
                    "<h1>Faucet is locked </h1>
                    <p> wait for: {} seconds</p>
                    <image src=\"/faucet/qrcode.png\"/>",
                    time
                ))));
            }
            Ok(Response::new(Body::from(format!(
                "<h1>Faucet is open </h1>
                <p> min withdrawable: {} Sats</p>
                <p> max withdrawable: {} Sats</p>
                <image src=\"/faucet/qrcode.png\"/>",
                faucet.amount_min_withdrawable / 1000,
                faucet.amount_max_withdrawable / 1000,
            ))))
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
