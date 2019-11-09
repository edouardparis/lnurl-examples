use hyper::client::HttpConnector;

pub struct Client {
    clt: hyper::Client<HttpConnector>,
    host: String,
    api_key: String,
}

impl Client {
    /// Creates a new client posted to a custom host.
    pub fn new(host: &str, apikey: &str) -> Client {
        Client {
            clt: hyper::Client::new(),
            api_key: apikey.to_string(),
            host: host.to_string(),
        }
    }
}
