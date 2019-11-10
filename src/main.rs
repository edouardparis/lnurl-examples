pub mod error;
pub mod front;
pub mod api;
pub mod faucet;
pub mod client;
pub mod lnurl;

use dotenv::dotenv;
use hyper::{Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use regex::Regex;
use std::sync::Arc;
use std::env;
use futures_util::future::join;

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = env::var("OPENNODE_API_KEY").expect("OPENNODE_API_KEY is needed");
    let api_url = env::var("API_URL").expect("API_URL is needed");
    let opennode_api_url = env::var("OPENNODE_API_URL").expect("OPENNODE_API_URL");

    // Construct our SocketAddr to listen on...
    let mut addr = ([127, 0, 0, 1], 8080).into();
    let environment = env::var("API_ENV").expect("API_ENV is needed");
    if environment == "production" {
        addr = ([0, 0, 0, 0], 8080).into();
    }

    lazy_static! {
        static ref API: Regex = Regex::new(r"/api/*").unwrap();
    }

    let client = client::Client::new(&opennode_api_url, &key);
    let faucet = Arc::new(faucet::Faucet::new(
            &format!("{}/api/lnurl/withdrawal", api_url),
            &format!("{}/api/withdrawals/create", api_url),
            client, 10
    ));

    let faucet_service = faucet::start(Arc::clone(&faucet));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(move |_| {
        let faucet = Arc::clone(&faucet);
        async move {
            Ok::<_, error::GenericError>(service_fn(move |req| {
                let faucet = Arc::clone(&faucet);
                async move {
                    info!("{} {}", req.method(), req.uri().path());
                    if API.is_match(req.uri().path()) {
                        return handle(api::routes(req, faucet).await)
                    }
                    handle(front::routes(req, faucet))
                }
            }))
        }
    });

    env_logger::init();
    info!("starting server :8080");

    // Then bind and serve...
    let server = Server::bind(&addr)
        .serve(make_service);

    // Finally, spawn `server` onto an Executor...
    let (s_res, _) = join(server, faucet_service).await;
    if let Err(e)  = s_res {
        eprintln!("server error: {}", e);
    }
}

pub fn handle(res: Result<Response<Body>, error::GenericError>)
    -> Result<Response<Body>, error::GenericError> {
        res.map_err(|e| {
            error!("{}", e);
            e
        }).and_then(|res| {
            info!("{}", res.status());
            Ok(res)
        })
}
