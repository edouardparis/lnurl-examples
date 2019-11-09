use crate::client::Client;
use std::sync::atomic::AtomicUsize;

pub struct Faucet {
    pub limit: AtomicUsize,
    pub counter: AtomicUsize,
    pub url: String,
    pub client: Client,
}

impl Faucet {
    pub fn new(url: &str, clt: Client, limit: AtomicUsize) -> Faucet {
        Faucet {
            limit: limit,
            counter: AtomicUsize::new(0),
            url: url.to_string(),
            client: clt,
        }
    }
}
