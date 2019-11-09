pub mod error;
pub mod front;
pub mod api;
pub mod faucet;
pub mod client;

use dotenv::dotenv;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use regex::Regex;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Construct our SocketAddr to listen on...
    let addr = ([127, 0, 0, 1], 8080).into();

    lazy_static! {
        static ref API: Regex = Regex::new(r"/api/*").unwrap();
    }

    let client = client::Client::new("hello", "hello");
    let _faucet = faucet::Faucet::new("hello", client);

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_| async {
        Ok::<_, error::GenericError>(service_fn(move |req| async {
            if API.is_match(req.uri().path()) {
                return api::routes(req)
            }
            front::routes(req)
        }))
    });

    env_logger::init();
    info!("starting server :8080");

    // Then bind and serve...
    let server = Server::bind(&addr)
        .serve(make_service);

    // Finally, spawn `server` onto an Executor...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
