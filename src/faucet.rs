use crate::client::Client;

pub struct Faucet {
    pub url: String,
    pub client: Client,
}

impl Faucet {
    pub fn new(url: &str, clt: Client) -> Faucet {
        Faucet {
            url: url.to_string(),
            client: clt,
        }
    }
}
