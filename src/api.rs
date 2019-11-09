use crate::error;
use hyper::{Body, Error, Method, Request, Response, Server, StatusCode};

static INDEX: &[u8] = b"<h1> Welcome to api !</h1>";
static NOTFOUND: &[u8] = b"Not Found";

pub fn routes(req: Request<Body>) -> Result<Response<Body>, error::GenericError> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/api/hello") => Ok(Response::new(INDEX.into())),
        _ => {
            // Return 404 not found response.
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(NOTFOUND.into())
                .unwrap())
        }
    }
}
